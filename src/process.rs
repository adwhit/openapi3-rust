use ::{Map, OpenApi};
use objects::*;
use regex::Regex;

pub struct Entrypoint {
    pub route: String,
    pub method: Method,
    pub args: Vec<Arg>,
    pub response: Response
}

pub struct Arg {
    pub name: String,
    pub type_: NativeType,
    pub location: Location
}

pub struct Response {
    fake: u32
}

pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete
}

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
}

pub fn flatten(spec: &OpenApi) -> Vec<Entrypoint> {
    let entrypoints = Vec::new();
    let mut components = &Default::default();
    components = spec.components.as_ref().unwrap_or(components);
    for (route, path) in &spec.paths {
    }
    entrypoints
}

fn inspect_route(route: &str) -> Vec<String> {
    let re = Regex::new(r"^\{(.+)\}$").unwrap();
    route.split("/")
        .filter_map(|section| re.captures(section))
        .map(|c| c.get(1).unwrap().as_str().into())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_inspect_route() {
        let res = inspect_route("/pets/{petId}/name/{petName}/x{bogus}x");
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], "petId");
        assert_eq!(res[1], "petName");
    }
}
