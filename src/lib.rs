#[macro_use]
extern crate error_chain;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use std::fs::File;
use std::io::Read;
use std::collections::BTreeMap;
//TODO use std::rc::Rc;

pub use serde_yaml::Value as YamlValue;
pub use errors::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybeRef<T> {
    Concrete(T),
    Ref(Ref),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ref {
    #[serde(rename = "$ref")]
    ref_: String,
}

mod errors {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
            Yaml(::serde_yaml::Error);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpenApi {
    pub openapi: String,
    pub info: Info,
    pub servers: Option<Vec<Server>>,
    pub paths: BTreeMap<String, Path>,
    pub components: Option<Components>,
    pub security: Option<SecurityRequirement>,
    pub tags: Option<Tag>,
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocs>,
}

impl OpenApi {
    pub fn from_reader<R: Read>(reader: R) -> Result<OpenApi> {
        Ok(serde_yaml::from_reader(reader)?)
    }

    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<OpenApi> {
        let file = File::open(path)?;
        OpenApi::from_reader(file)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Info {
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "termsOfService")]
    pub terms_of_service: Option<String>,
    pub contact: Option<Contact>,
    pub license: Option<License>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Contact {
    pub name: Option<String>,
    pub url: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct License {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Server {
    pub url: String,
    pub description: Option<String>,
    pub variables: Option<BTreeMap<String, ServerVariable>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServerVariable {
    #[serde(rename = "enum")]
    pub enum_: Vec<String>,
    pub default: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tag {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocs>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Path {
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub get: Option<Operation>,
    pub put: Option<Operation>,
    pub post: Option<Operation>,
    pub delete: Option<Operation>,
    pub options: Option<Operation>,
    pub head: Option<Operation>,
    pub patch: Option<Operation>,
    pub trace: Option<Operation>,
    pub servers: Option<Vec<Server>>,
    pub parameters: Option<Vec<MaybeRef<Parameter>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Operation {
    pub tags: Option<Vec<String>>,
    pub summary: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocs>,
    #[serde(rename = "operationId")]
    pub operation_id: Option<String>,
    pub parameters: Option<Vec<MaybeRef<Parameter>>>,
    #[serde(rename = "requestBody")]
    pub request_body: Option<MaybeRef<RequestBody>>,
    pub responses: BTreeMap<String, MaybeRef<Response>>,
    pub callbacks: Option<BTreeMap<String, MaybeRef<Callback>>>,
    #[serde(default)]
    pub deprecated: bool,
    pub security: Option<Vec<SecurityRequirement>>,
    pub servers: Option<Vec<Server>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestBody {
    pub description: Option<String>,
    pub content: BTreeMap<String, MediaType>,
    #[serde(default)]
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "in")]
    pub in_: Location,
    pub description: Option<String>,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub deprecated: bool,
    #[serde(rename = "allowEmptyValue")]
    #[serde(default)]
    pub allow_empty_value: bool,

    pub style: Option<Style>,
    #[serde(default)]
    pub explode: bool,
    #[serde(rename = "allowReserved")]
    #[serde(default)]
    pub allow_reserved: bool,
    pub schema: MaybeRef<Schema>,
    pub example: Option<YamlValue>,
    pub examples: Option<BTreeMap<String, MaybeRef<Example>>>,

    pub content: Option<BTreeMap<String, MediaType>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Response {
    pub description: String,
    pub headers: Option<BTreeMap<String, MaybeRef<Header>>>,
    pub content: Option<BTreeMap<String, MediaType>>,
    pub links: Option<BTreeMap<String, MaybeRef<Link>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MediaType {
    pub schema: Option<MaybeRef<Schema>>,
    pub example: Option<YamlValue>,
    pub examples: Option<BTreeMap<String, MaybeRef<Example>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Schema {
    pub required: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub type_: Option<Type>,
    pub format: Option<Format>,
    pub properties: Option<BTreeMap<String, Box<MaybeRef<Schema>>>>,
    pub items: Option<Box<MaybeRef<Schema>>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Components {
    schemas: Option<BTreeMap<String, MaybeRef<Schema>>>,
    responses: Option<BTreeMap<String, MaybeRef<Response>>>,
    parameters: Option<BTreeMap<String, MaybeRef<Parameter>>>,
    examples: Option<BTreeMap<String, MaybeRef<Example>>>,
    #[serde(rename = "requestBodies")]
    request_bodies: Option<BTreeMap<String, MaybeRef<RequestBody>>>,
    headers: Option<BTreeMap<String, MaybeRef<Header>>>,
    #[serde(rename = "securitySchemes")]
    security_schemes: Option<BTreeMap<String, MaybeRef<SecurityScheme>>>,
    links: Option<BTreeMap<String, MaybeRef<Link>>>,
    callbacks: Option<BTreeMap<String, MaybeRef<Callback>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Example {
    summary: Option<String>,
    description: Option<String>,
    value: YamlValue,
    #[serde(rename = "externalValue")]
    external_value: Option<String>,
}

pub type Callback = BTreeMap<String, Path>;

type SecurityRequirement = YamlValue;
type ExternalDocs = YamlValue;
type Header = YamlValue;
type Link = YamlValue;
type SecurityScheme = YamlValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Location {
    Path,
    Query,
    Header,
    Cookie,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Boolean,
    Object,
    Array,
    Number,
    String,
    Integer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    Int32,
    Int64,
    Float,
    Double,
    Byte,
    Binary,
    Date,
    #[serde(rename = "date-time")]
    DateTime,
    Password,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Style {
    Form,
    Simple,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_petstore() {
        let file = File::open("test_apis/petstore.yaml").unwrap();
        let api: OpenApi = match OpenApi::from_reader(file) {
            Ok(api) => api,
            Err(e) => panic!("{}", e),
        };
        println!("{:#?}", api)
    }

    #[test]
    fn parse_petstore_expanded() {
        let file = File::open("test_apis/petstore-expanded.yaml").unwrap();
        let api: OpenApi = match OpenApi::from_reader(file) {
            Ok(api) => api,
            Err(e) => panic!("{}", e),
        };
        println!("{:#?}", api)
    }

    #[test]
    fn parse_uber() {
        let file = File::open("test_apis/uber.yaml").unwrap();
        let api: OpenApi = match OpenApi::from_reader(file) {
            Ok(api) => api,
            Err(e) => panic!("{}", e),
        };
        println!("{:#?}", api)
    }
}
