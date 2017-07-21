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
                                .and_then(|maybe| NativeType::from_json_schema(maybe, components))
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
                    NativeType::from_json_schema(&parameter.schema, components).map(|native_type| {
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
    Date,
    DateTime,
    String,
    Struct(Map<NativeType>),
    Vec(Box<NativeType>),
    Option(Box<NativeType>),
}

impl NativeType {
    fn from_format(format: Format) -> NativeType {
        use Format::*;
        match format {
            Int32 => NativeType::I32,
            Int64 => NativeType::I64,
            Float => NativeType::F32,
            Double => NativeType::F64,
            Byte => NativeType::F64,
            Date => NativeType::Date,
            DateTime => NativeType::DateTime,
            Binary => NativeType::String,
            Password => NativeType::String,
        }
    }

    fn from_json_schema(maybe_schema: &MaybeRef<Schema>, components: &Components) -> Result<Self> {
        // let schema = maybe_schema.resolve_ref_opt(components.schema)?;
        // if schema.properties.is_none() && schema.type_.is_none() {
        //     bail!("No type specified")
        // };
        // let type_ = schema.type_.unwrap_or(Type::Object);
        // if let Some(f) = schema.format {
        //     if f.compatible_with_type(type_) {
        //         bail!("Type {:?} and Format {:?} are not compatible")
        //     }
        // }
        // match type_ {
        //     Boolean => NativeType::Bool,
        //     Number => NativeType::F64,
        //     Integer => NativeType::I64,
        //     String => NativeType::String,
        //     Object => match schema.properties {
        //         None => bail!("No properties for object definition"),
        //         Some(props) => build_struct_from_properties(props, componenets)
        //     }
        //     Array => {
        //         schema.items
        //     }
        // }
        unimplemented!()
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

impl Schema {
    fn schemafy(&self, name: &str) -> Result<String> {
        let json = to_json_string(self)?;
        schemafy::generate(Some(name), &json).map_err(|e| format!("Schemafy error: {}", e).into())
    }
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
        let schema = Schema {
            type_: Some(Type::Integer),
            ..Default::default()
        };
        let outcome = schema.schemafy("my dummy type").unwrap();
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
        let schema = schema.schemafy("Pet").unwrap();
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
        let schema = schema.schemafy("Pets").unwrap();
        assert!(schema.contains("pub type Pets = Vec<Pet>;"));
    }
}
