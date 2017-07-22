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

pub fn extract_entrypoints(spec: &OpenApi) -> Vec<Entrypoint> {
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

fn build_entrypoint(
    route: String,
    method: Method,
    operation: &Operation,
    components: &Components,
) -> Result<Entrypoint> {
    // TODO verify route args
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

#[derive(Debug, Clone, new)]
pub struct Arg {
    pub name: String,
    pub type_: NativeType,
    pub location: Location,
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
                    let required = parameter.required.unwrap_or(false);
                    NativeType::from_json_schema(&parameter.schema, required).map(|native_type| {
                        Arg::new(parameter.name.clone(), native_type, parameter.in_)
                    })
                },
            )
        })
        .collect()
}

#[derive(Debug, Default, Clone, new)]
pub struct Response {
    pub status_code: String,
    pub return_type: Option<NativeType>,
    pub content_type: Option<String>,
}

impl Response {
    fn build_from_response_obj(
        status_code: String,
        response_obj: &ResponseObj,
    ) -> Result<Response> {
        match response_obj.content {
            None => return Ok(Response::new(status_code, None, None)), // No data returned
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
                                // For responses, the default required state is 'true'
                                .and_then(|maybe| NativeType::from_json_schema(maybe, true))
                                .map(|typ| {
                                    Response::new(
                                        status_code,
                                        Some(typ),
                                        Some(content_type.clone()),
                                    )
                                })
                    })
            }
        }
    }
}

fn build_responses(operation: &Operation, components: &Components) -> Vec<Result<Response>> {
    operation
        .responses
        .iter()
        .map(|(code, maybe)| {
            let response_obj = maybe.resolve_ref_opt(&components.responses)?;
            Response::build_from_response_obj(code.clone(), response_obj)
        })
        .collect()
}

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NativeType {
    I32,
    I64,
    F32,
    F64,
    Bool,
    String,
    Named(String),
    Array(Vec<NativeType>),
    Option(Box<NativeType>),
    Anonymous(Box<Schema>),
}

impl NativeType {
    fn from_json_schema(schema: &Schema, required: bool) -> Result<Self> {
        let out = if let Some(ref ref_) = schema.ref_ {
            // If the schema is a reference, grab the name
            match ref_.rfind("/") {
                None => bail!("Reference {} is not valid path", ref_),
                Some(loc) => {
                    let refname = ref_.split_at(loc + 1).1;
                    NativeType::Named(refname.into())
                }
            }
        } else {
            match schema.type_.len() {
                0 => NativeType::Anonymous(Box::new(schema.clone())), // assume it is an object
                1 => {
                    // If the type is a primitive, pluck it from the schema
                    // Otherwise, return the schema
                    use schemafy::schema::SimpleTypes::*;
                    match *(schema.type_.first().unwrap()) {
                        Object => NativeType::Anonymous(Box::new(schema.clone())),
                        Boolean => NativeType::Bool,
                        Integer => NativeType::I64,
                        Null => bail!("Null is not valid as per spec"),
                        Number => NativeType::F64,
                        String => NativeType::String,
                        Array => {
                            if schema.items.len() == 0 {
                                bail!("Items missing for array schema")
                            }
                            let natives = schema
                                .items
                                .iter()
                                .map(|schema| NativeType::from_json_schema(schema, required))
                                .collect::<Result<Vec<_>>>()?;
                            NativeType::Array(natives)
                        }
                    }
                }
                other => bail!("Schema type is array of len {}", other),
            }
        };
        if !required {
            Ok(NativeType::Option(Box::new(out)))
        } else {
            Ok(out)
        }
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

fn extract_route_args(route: &str) -> BTreeSet<String> {
    let re = Regex::new(r"^\{(.+)\}$").unwrap();
    route
        .split("/")
        .filter_map(|section| re.captures(section))
        .map(|c| c.get(1).unwrap().as_str().into())
        .collect()
}

pub fn schema_to_code(name: &str, schema: &Schema) -> Result<String> {
    schemafy::generate(Some(name), &to_json_string(schema)?)
        .map_err(|e| format!("Schemafy failed: {}", e).into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_extract_route_args() {
        let res = extract_route_args("/pets/{petId}/name/{petName}/x{bogus}x");
        assert_eq!(res.len(), 2);
        assert!(res.contains("petId"));
        assert!(res.contains("petName"));
    }

    #[test]
    fn test_extract_entrypoints() {
        let yaml = include_str!("../test_specs/petstore.yaml");
        let api = OpenApi::from_string(yaml).unwrap();
        let flat = extract_entrypoints(&api);
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
            .unwrap(); // yuck
        let native = NativeType::from_json_schema(&schema, true).unwrap();
        // TODO: this would be easier if Schema had a default impl
        let expectstr = r#"{
            "required": [ "id", "name" ],
            "properties": {
                "id": { "type": "integer", "format": "int64" },
                "name": { "type": "string" },
                "tag": { "type": "string" }
            }
        }"#;
        let expect_schema: Schema = serde_json::from_str(expectstr).unwrap();
        assert_eq!(native, NativeType::Anonymous(Box::new(expect_schema)));
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
            .unwrap(); // yuck
        let native = NativeType::from_json_schema(&schema, true).unwrap();
        let expect = NativeType::Array(vec![NativeType::Named("Pet".into())]);
        assert_eq!(native, expect);
    }
}
