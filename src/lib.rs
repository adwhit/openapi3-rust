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
    pub security: Option<Security>,
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
    pub parameters: Option<Vec<ParamOrRef>>
}

type Operation = YamlValue;
type ParamOrRef = YamlValue;
type Security = YamlValue;
type ExternalDocs = YamlValue;
type Components = YamlValue;

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
