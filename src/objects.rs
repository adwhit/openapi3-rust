pub use serde_yaml::Value as YamlValue;
use {MaybeRef, Map, MapMaybeRef};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ref {
    #[serde(rename = "$ref")]
    pub ref_: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Info {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "termsOfService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Contact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct License {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Server {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<Map<ServerVariable>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServerVariable {
    #[serde(rename = "enum")]
    pub enum_: Vec<String>,
    pub default: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tag {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "externalDocs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocs>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Path {
    #[serde(rename = "$ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<Server>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<MaybeRef<Parameter>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Operation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "externalDocs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocs>,
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<MaybeRef<Parameter>>>,
    #[serde(rename = "requestBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<MaybeRef<RequestBody>>,
    pub responses: MapMaybeRef<ResponseObj>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callbacks: Option<MapMaybeRef<Callback>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<SecurityRequirement>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<Server>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub content: Map<MediaType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "in")]
    pub in_: Location,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(rename = "allowEmptyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_empty_value: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<Style>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explode: Option<bool>,
    #[serde(rename = "allowReserved")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_reserved: Option<bool>,
    pub schema: MaybeRef<Schema>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<YamlValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<MapMaybeRef<Example>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Map<MediaType>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResponseObj {
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<MapMaybeRef<Header>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Map<MediaType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<MapMaybeRef<Link>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct MediaType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<MaybeRef<Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<YamlValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<MapMaybeRef<Example>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Schema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<Format>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Map<Box<MaybeRef<Schema>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<MaybeRef<Schema>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Components {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<MapMaybeRef<Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<MapMaybeRef<ResponseObj>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<MapMaybeRef<Parameter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<MapMaybeRef<Example>>,
    #[serde(rename = "requestBodies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_bodies: Option<MapMaybeRef<RequestBody>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<MapMaybeRef<Header>>,
    #[serde(rename = "securitySchemes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_schemes: Option<MapMaybeRef<SecurityScheme>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<MapMaybeRef<Link>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callbacks: Option<MapMaybeRef<Callback>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Example {
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    value: YamlValue,
    #[serde(rename = "externalValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    external_value: Option<String>,
}

pub type Callback = Map<Path>;
pub type SecurityRequirement = YamlValue;
pub type ExternalDocs = YamlValue;
pub type Header = YamlValue;
pub type Link = YamlValue;
pub type SecurityScheme = YamlValue;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Location {
    Path,
    Query,
    Header,
    Cookie,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Boolean,
    Object,
    Array,
    Number,
    String,
    Integer,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

impl Format {
    pub fn compatible_with_type(self, type_: Type) -> bool {
        use Type::*;
        use Format::*;
        match (type_, self) {
            (Number, Float) |
            (Number, Double) |
            (Integer, Int32) |
            (Integer, Int64) |
            (String, Byte) |
            (String, Binary) |
            (String, Date) |
            (String, DateTime) |
            (String, Password) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Style {
    Form,
    Simple,
}
