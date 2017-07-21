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
    pub variables: Option<Map<ServerVariable>>,
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
    pub responses: MapMaybeRef<ResponseObj>,
    pub callbacks: Option<MapMaybeRef<Callback>>,
    #[serde(default)]
    pub deprecated: bool,
    pub security: Option<Vec<SecurityRequirement>>,
    pub servers: Option<Vec<Server>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestBody {
    pub description: Option<String>,
    pub content: Map<MediaType>,
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
    pub examples: Option<MapMaybeRef<Example>>,

    pub content: Option<Map<MediaType>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResponseObj {
    pub description: String,
    pub headers: Option<MapMaybeRef<Header>>,
    pub content: Option<Map<MediaType>>,
    pub links: Option<MapMaybeRef<Link>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct MediaType {
    pub schema: Option<MaybeRef<Schema>>,
    pub example: Option<YamlValue>,
    pub examples: Option<MapMaybeRef<Example>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Schema {
    pub required: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub type_: Option<Type>,
    pub format: Option<Format>,
    pub properties: Option<Map<Box<MaybeRef<Schema>>>>,
    pub items: Option<Box<MaybeRef<Schema>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Components {
    pub schemas: Option<MapMaybeRef<Schema>>,
    pub responses: Option<MapMaybeRef<ResponseObj>>,
    pub parameters: Option<MapMaybeRef<Parameter>>,
    pub examples: Option<MapMaybeRef<Example>>,
    #[serde(rename = "requestBodies")]
    pub request_bodies: Option<MapMaybeRef<RequestBody>>,
    pub headers: Option<MapMaybeRef<Header>>,
    #[serde(rename = "securitySchemes")]
    pub security_schemes: Option<MapMaybeRef<SecurityScheme>>,
    pub links: Option<MapMaybeRef<Link>>,
    pub callbacks: Option<MapMaybeRef<Callback>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Example {
    summary: Option<String>,
    description: Option<String>,
    value: YamlValue,
    #[serde(rename = "externalValue")]
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
