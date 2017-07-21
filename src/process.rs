use {OpenApi, Result, Map, MaybeRef};
use objects::*;
use errors::ErrorKind;
use regex::Regex;
use serde_json::to_string as to_json_string;
use schemafy;
use std::collections::{BTreeSet, BTreeMap};

#[derive(Debug, Clone, new)]
pub struct Entrypoint {
    pub route: String,
    pub method: Method,
    pub args: Vec<Arg>,
    pub responses: Vec<Response>,
    pub operation_id: String,
}

fn build_entrypoint(
    route: String,
    method: Method,
    operation: &Operation,
    components: &Components,
) -> Result<Entrypoint> {
    let route_args = extract_route_args(&route);
    let args = build_args(operation, components)?;
    let responses = build_responses(operation, components);
    let responses = responses
        .into_iter()
        .filter_map(|res| match res {
            Ok(resp) => Some(resp),
            Err(e) => {
                eprintln!("{}", e);
                None
            }
        })
        .collect();
    let operation_id = operation
        .operation_id
        .as_ref()
        .ok_or(ErrorKind::from("No operation_id found"))?;
    Ok(Entrypoint::new(
        route,
        method,
        args,
        responses,
        operation_id.clone(),
    ))
}

fn build_responses(operation: &Operation, components: &Components) -> Vec<Result<Response>> {
    operation
        .responses
        .iter()
        .map(|(code, maybe)| {
            let response_obj = maybe.resolve_ref_opt(&components.responses)?;
            match response_obj.content {
                None => return Ok(Response::new(code.clone(), None, None)), // No data returned
                Some(ref content_map) => {
                    content_map
                        .iter()
                        .next()
                        .ok_or("Content map empty".into())
                        .and_then(|(content_type, media)| {
                            media
                                .schema
                                .as_ref()
                                .ok_or("Media schema not found".into())
                                .and_then(|maybe| NativeType::from_json_schema(maybe))
                                .map(|typ| {
                                    Response::new(
                                        code.clone(),
                                        Some(typ),
                                        Some(content_type.clone()),
                                    )
                                })
                        })
                }
            }
        })
        .collect()
}

fn build_args(operation: &Operation, components: &Components) -> Result<Vec<Arg>> {
    let op_parameters = match operation.parameters.as_ref() {
        Some(p) => p,
        None => return Ok(Vec::new()),
    };
    op_parameters
        .iter()
        .map(|maybe| {
            maybe.resolve_ref_opt(&components.parameters).and_then(
                |parameter| {
                    NativeType::from_json_schema(&parameter.schema).map(|native_type| {
                        Arg::new(parameter.name.clone(), native_type, parameter.in_)
                    })
                },
            )
        })
        .collect()
}

#[derive(Debug, Clone, new)]
pub struct Arg {
    pub name: String,
    pub type_: NativeType,
    pub location: Location,
}

#[derive(Debug, Default, Clone, new)]
pub struct Response {
    pub status_code: String,
    pub return_type: Option<NativeType>,
    pub content_type: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum NativeType {
    I32,
    I64,
    F32,
    F64,
    Bool,
    String,
    Option(Box<NativeType>),
    Named(Option<String>),
}

impl NativeType {
    fn from_json_schema(maybe_schema: &MaybeRef<Schema>) -> Result<Self> {
        use schemafy::schema::SimpleTypes::*;
        let out = match *maybe_schema {
            MaybeRef::Concrete(ref schema) => {
                if schema.type_.len() != 1 {
                    bail!("Schema type is array")
                };
                match schema.type_[0] {
                    Array => NativeType::Named(None),
                    Boolean => NativeType::Bool,
                    Integer => NativeType::I64,
                    Null => bail!("Null is not valid as per spec"),
                    Number => NativeType::F64,
                    Object => NativeType::Named(None),
                    String => NativeType::String,
                }
            }
            MaybeRef::Ref(ref r) => {
                let name = match r.ref_.rfind("/") {
                    None => bail!("Reference {} is not valid path", r.ref_),
                    Some(loc) => r.ref_.split_at(loc + 1).0,
                };
                NativeType::Named(Some(name.into()))
            }
        };
        Ok(out)
    }
}

impl Path {
    fn as_map(&self) -> BTreeMap<Method, &Operation> {
        use self::Method::*;
        let mut map = BTreeMap::new();
        if let Some(ref op) = self.get {
            map.insert(Get, op);
        }
        if let Some(ref op) = self.post {
            map.insert(Post, op);
        }
        if let Some(ref op) = self.put {
            map.insert(Put, op);
        }
        if let Some(ref op) = self.patch {
            map.insert(Patch, op);
        }
        if let Some(ref op) = self.delete {
            map.insert(Delete, op);
        }
        map
    }
}

pub fn flatten(spec: &OpenApi) -> Vec<Entrypoint> {
    let mut out = Vec::new();
    let mut components = &Default::default();
    components = spec.components.as_ref().unwrap_or(components);
    for (route, path) in &spec.paths {
        for (method, op) in path.as_map() {
            match build_entrypoint(route.clone(), method, op, components) {
                Ok(entrypoint) => out.push(entrypoint),
                Err(e) => eprintln!("{}", e),
            }
        }
    }
    out
}

fn extract_route_args(route: &str) -> BTreeSet<String> {
    let re = Regex::new(r"^\{(.+)\}$").unwrap();
    route
        .split("/")
        .filter_map(|section| re.captures(section))
        .map(|c| c.get(1).unwrap().as_str().into())
        .collect()
}

fn schema_to_string(name: &str, schema: &Schema) -> Result<String> {
    schemafy::generate(Some(name), &to_json_string(schema)?)
        .map_err(|e| format!("Schemafy failed: {}", e).into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_extract_route_args() {
        let res = extract_route_args("/pets/{petId}/name/{petName}/x{bogus}x");
        assert_eq!(res.len(), 2);
        assert!(res.contains("petId"));
        assert!(res.contains("petName"));
    }

    #[test]
    fn test_flatten() {
        let file = File::open("test_specs/petstore.yaml").unwrap();
        let api = OpenApi::from_reader(file).unwrap();
        let flat = flatten(&api);
        println!("{:#?}", flat);
        assert_eq!(flat.len(), 3);
    }

    #[test]
    fn test_atom_schemafy() {
        let schema = r#"{"type": "integer"}"#;
        let outcome = schemafy::generate(Some("my dummy type"), schema).unwrap();
        println!("{}", outcome);
        assert!(outcome.contains("MyDummyType = i64"));
    }

    #[test]
    fn test_simple_schemafy() {
        let yaml = include_str!("../test_specs/petstore.yaml");
        let api = OpenApi::from_string(yaml).unwrap();
        let schema: &Schema = api.components
            .as_ref()
            .unwrap()
            .schemas
            .as_ref()
            .unwrap()
            .get("Pet")
            .as_ref()
            .map(|schema| schema.as_result().unwrap())
            .unwrap(); // yuck
        let schema = schema_to_string("Pet", schema).unwrap();
        assert!(schema.contains("pub struct Pet"));
        assert!(schema.contains("pub id"));
        assert!(schema.contains("pub name"));
        assert!(schema.contains("pub tag"));
    }

    #[test]
    fn test_referenced_schemafy() {
        let yaml = include_str!("../test_specs/petstore.yaml");
        let api = OpenApi::from_string(yaml).unwrap();
        let schema: &Schema = api.components
            .as_ref()
            .unwrap()
            .schemas
            .as_ref()
            .unwrap()
            .get("Pets")
            .as_ref()
            .map(|schema| schema.as_result().unwrap())
            .unwrap(); // yuck
        let schema = schema_to_string("Pets", schema).unwrap();
        assert!(schema.contains("pub type Pets = Vec<Pet>;"));
    }
}
