use ::{Map, MapMaybeRef, OpenApi};
use objects::*;
use regex::Regex;
use std::collections::{BTreeSet, BTreeMap};

#[derive(Debug, Clone, new)]
pub struct Entrypoint {
    pub route: String,
    pub method: Method,
    pub args: Vec<Arg>,
    pub response: Reply,
    pub operation_id: String
}

fn build_entrypoint(route: String, method: Method,
                    operation: &Operation, components: &Components) -> Entrypoint {
    let route_args = extract_route_args(&route);
    let args = build_args(operation, components);
    //let response = 
    unimplemented!();
    //Entrypoint::new(route, method, args, response, operation_id)

}

fn build_args(operation: &Operation, components: &Components) -> Vec<Arg> {
    let mut param_refs = &Default::default();
    param_refs = components.parameters.as_ref().unwrap_or(&param_refs);
    operation.parameters.as_ref().map(|params| {
        params.iter()
            .flat_map(|maybe| maybe.resolve_ref(param_refs))
            .flat_map(|parameter| {
                let dummy_schema_ref = &Default::default();
                parameter.schema
                    .resolve_ref(components.schemas.as_ref().unwrap_or(dummy_schema_ref))
                    .and_then(|schema| {
                        NativeType::from_openapi_type_or_format(&schema.type_, &schema.format)
                    }).map(|native_type| {
                        Arg::new(parameter.name.clone(),
                                 native_type,
                                 parameter.in_)
                    })
            }).collect()
    }).unwrap_or(Vec::new())
}

#[derive(Debug, Clone, new)]
pub struct Arg {
    pub name: String,
    pub type_: NativeType,
    pub location: Location
}

#[derive(Debug, Clone, new)]
pub struct Reply {
    pub type_: NativeType
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
    let mut entrypoints = Vec::new();
    let mut components = &Default::default();
    components = spec.components.as_ref().unwrap_or(components);
    for (route, path) in &spec.paths {
        for (method, op) in path.as_map() {
            let entrypoint = build_entrypoint(route.clone(), method, op, components);
            entrypoints.push(entrypoint);
        }
    }
    entrypoints
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
    #[test]
    fn test_extract_route_args() {
        let res = inspect_route("/pets/{petId}/name/{petName}/x{bogus}x");
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], "petId");
        assert_eq!(res[1], "petName");
    }
}
