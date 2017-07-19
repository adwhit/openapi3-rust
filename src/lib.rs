#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;
#[macro_use]
extern crate error_chain;

use std::fs::File;
use std::io::Read;
use std::collections::BTreeMap;
use serde_yaml::Value as YamlValue;
use errors::*;

mod errors {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
            Yaml(::serde_yaml::Error);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApi {
    pub openapi: String,
    pub info: Info,
    pub servers: Option<Vec<Server>>,
    pub paths: BTreeMap<String, Path>,
    pub components: Option<Components>,
    pub security: Option<SecurityRequirement>,
    pub tags: Option<Tag>,
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocs>
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
pub struct Info {
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "termsOfService")]
    pub terms_of_service: Option<String>,
    pub contact: Option<Contact>,
    pub license: Option<License>,
    pub version: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub name: Option<String>,
    pub url: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub name: String,
    pub url: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub url: String,
    pub description: Option<String>,
    pub variables: Option<BTreeMap<String, ServerVariable>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerVariable {
    #[serde(rename = "enum")]
    pub enum_: Vec<String>,
    pub default: String,
    pub description: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocs>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub parameters: Option<Vec<ParameterOrRef>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub tags: Option<Vec<String>>,
    pub summary: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocs>,
    #[serde(rename = "operationId")]
    pub operation_id: Option<String>,
    pub parameters: Option<Vec<ParameterOrRef>>,
    #[serde(rename = "requestBody")]
    pub request_body: Option<RequestBodyOrRef>,
    pub responses: BTreeMap<String, ResponseOrRef>,
    pub callbacks: Option<BTreeMap<String, CallbackOrRef>>,
    pub deprecated: Option<bool>,
    pub security: Option<Vec<SecurityRequirement>>,
    pub servers: Option<Vec<Server>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestBodyOrRef {
    RequestBody(RequestBody),
    Ref(Ref)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestBody {
    pub description: Option<String>,
    pub content: BTreeMap<String, MediaType>,
    pub required: Option<bool>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParameterOrRef {
    Parameter(Parameter),
    Ref(Ref)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    name: String,
    #[serde(rename = "in")]
    in_: String,
    description: Option<String>,
    required: Option<bool>,
    deprecated: Option<bool>,
    #[serde(rename = "allowEmptyValue")]
    allow_empty_value: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseOrRef {
    Response(Response),
    Ref(Ref)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub description: String,
    pub headers: Option<BTreeMap<String, HeaderOrRef>>,
    pub content: Option<BTreeMap<String, MediaType>>,
    pub links: Option<BTreeMap<String, LinkOrRef>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaType {
    pub schema: Option<SchemaOrRef>,
    pub example: Option<YamlValue>,
    pub examples: Option<BTreeMap<String, ExampleOrRef>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchemaOrRef {
    Schema(Schema),
    Ref(Ref)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub required: Option<Vec<String>>,
    pub type_: Option<String>,
    pub format: Option<String>,
    pub properties: Option<BTreeMap<String, Box<Schema>>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ref {
    #[serde(rename = "$ref")]
    ref_: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Components {
    schemas: Option<BTreeMap<String, SchemaOrRef>>,
    responses: Option<BTreeMap<String, ResponseOrRef>>,
    parameters: Option<BTreeMap<String, ParameterOrRef>>,
    examples: Option<BTreeMap<String, ExampleOrRef>>,
    #[serde(rename = "requestBodies")]
    request_bodies: Option<BTreeMap<String, RequestBodyOrRef>>,
    headers: Option<BTreeMap<String, HeaderOrRef>>,
    #[serde(rename = "securitySchemes")]
    security_schemes: Option<BTreeMap<String, SecuritySchemeOrRef>>,
    links: Option<BTreeMap<String, LinkOrRef>>,
    callbacks: Option<BTreeMap<String, CallbackOrRef>>,
}

type SecuritySchemeOrRef = YamlValue;
type ExampleOrRef = YamlValue;
type HeaderOrRef = YamlValue;
type LinkOrRef = YamlValue;
type CallbackOrRef = YamlValue;
type SecurityRequirement = YamlValue;
type ExternalDocs = YamlValue;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_petstore() {
        let file = File::open("test_apis/petstore.yaml").unwrap();
        let api: OpenApi = match OpenApi::from_reader(file) {
            Ok(api) => api,
            Err(e) => panic!("{}", e)
        };
        println!("{:#?}", api)
    }
}
