use ::{Map, MapMaybeRef, OpenApi, Result};
use objects::{Operation, Components, Location, Type, Format, Schema, Path};
use errors::ErrorKind;
use regex::Regex;
use std::collections::{BTreeSet, BTreeMap};

#[derive(Debug, Clone, new)]
pub struct Entrypoint {
    pub route: String,
    pub method: Method,
    pub args: Vec<Arg>,
    pub response: Response,
    pub operation_id: String
}

fn build_entrypoint(route: String, method: Method,
                    operation: &Operation, components: &Components) -> Result<Entrypoint> {
    let route_args = extract_route_args(&route);
    let args = build_args(operation, components)?;
    let response = build_responses(operation, components)?;
    let operation_id = operation.operation_id.as_ref()
        .ok_or(ErrorKind::from("No operation_id found"))?;
    Ok(Entrypoint::new(route, method, args, response, operation_id.clone()))
}

fn build_responses(operation: &Operation, components: &Components) -> Result<Response> {
    operation.responses.get("200")
        .ok_or(ErrorKind::from("200 not found"))
        .and_then(|maybe| {
            match components.responses {
                Some(ref resp_ref) => maybe.resolve_ref(resp_ref),
                None => maybe.as_option()
            }.ok_or(ErrorKind::from("Response object not resolved"))
        })
        .and_then(|response_obj| response_obj.content.as_ref()
                  .ok_or(ErrorKind::from("Content not found")))
        .and_then(|content_map| content_map.iter().next()
                  .ok_or(ErrorKind::from("Content map empty")))
        .and_then(|(content_type, media)| media.schema.as_ref()
                  .ok_or(ErrorKind::from("Media schema not found")))
        .and_then(|maybe| {
            match components.schemas {
                Some(ref schema_ref) => maybe.resolve_ref(schema_ref),
                None => maybe.as_option()
            }.ok_or(ErrorKind::from("Media object not resolved"))
        })
        .and_then(|schema| {
            NativeType::from_schema(schema)
                .ok_or(ErrorKind::from("Media object not resolved"))
        })
        .map(|typ| Response::new(Some(typ)))
        .map_err(|e| e.into())
}

fn build_args(operation: &Operation, components: &Components) -> Result<Vec<Arg>> {
    let mut param_refs = &Default::default();
    param_refs = components.parameters.as_ref().unwrap_or(&param_refs);
    let args = operation.parameters.as_ref().map(|params| {
        params.iter()
            .filter_map(|maybe| maybe.resolve_ref(param_refs))
            .filter_map(|parameter| {
                // TODO is there a neater way to do this to get rid of dummy struct?
                let dummy_schema_ref = &Default::default();
                parameter.schema
                    .resolve_ref(components.schemas.as_ref().unwrap_or(dummy_schema_ref))
                    .and_then(NativeType::from_schema)
                    .map(|native_type| {
                        Arg::new(parameter.name.clone(),
                                 native_type,
                                 parameter.in_)
                    })
            }).collect()
    }).unwrap_or(Vec::new());
    Ok(args)
}

#[derive(Debug, Clone, new)]
pub struct Arg {
    pub name: String,
    pub type_: NativeType,
    pub location: Location
}

#[derive(Debug, Default, Clone, new)]
pub struct Response {
    pub type_: Option<NativeType>
}

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete
}

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum NativeType {
    I32,
    I64,
    F32,
    F64,
    Bool,
    Date,
    DateTime,
    String,
    // Struct(Map<NativeType>),
    // Vec(Box<NativeType>),
    // Option(Box<NativeType>),
    Struct,
    Vec,
}

impl NativeType {
    fn from_openapi_type(type_: Type) -> NativeType {
        use Type::*;
        match type_ {
            Boolean => NativeType::Bool,
            Object => NativeType::Struct,
            Array => NativeType::Vec,
            Number => NativeType::F64,
            Integer => NativeType::I64,
            String => NativeType::String,
        }
    }

    fn from_openapi_format(format: Format) -> NativeType {
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
            Password => NativeType::String
        }
    }

    fn from_openapi_type_or_format(type_: &Option<Type>, format: &Option<Format>) -> Option<Self> {
        // TODO check type and format agree
        match (*type_, *format) {
            (None, _) => None,
            (Some(atype), None) => Some(Self::from_openapi_type(atype)),
            (Some(_), Some(aformat)) => Some(Self::from_openapi_format(aformat))
        }
    }

    fn from_schema(schema: &Schema) -> Option<Self> {
        NativeType::from_openapi_type_or_format(&schema.type_, &schema.format)
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
                Err(e) => eprintln!("{}", e)
            }
        }
    }
    out
}

fn extract_route_args(route: &str) -> BTreeSet<String> {
    let re = Regex::new(r"^\{(.+)\}$").unwrap();
    route.split("/")
        .filter_map(|section| re.captures(section))
        .map(|c| c.get(1)
             .unwrap()
             .as_str()
             .into())
        .collect()
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
        let file = File::open("test_apis/petstore.yaml").unwrap();
        let api = OpenApi::from_reader(file).unwrap();
        let flat = flatten(&api);
        println!("{:#?}", flat);
    }
}
