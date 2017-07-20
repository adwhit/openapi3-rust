use ::{Map, Type, Format, Location, OpenApi, Components};

struct Entrypoint {
    route: String,
    method: Method,
    args: Vec<Arg>,
    response: Response
}

struct Arg {
    name: String,
    type_: NativeType,
    location: Location
}

struct Response {
    fake: u32
}

enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete
}

enum NativeType {
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

fn flatten(spec: &OpenApi) -> Vec<Entrypoint> {
    let entrypoints = Vec::new();
    let mut components = &Default::default();
    components = spec.components.as_ref().unwrap_or(components);
    for (route, path) in spec.paths {
    }
    entrypoints
}

fn inspect_route(route: &str) -> (String, Vec<String>) {
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_inspect_route() {
    }
}
