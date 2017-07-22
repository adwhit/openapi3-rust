#[macro_use]
extern crate error_chain;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate serde_json;
extern crate schemafy;
extern crate regex;

use std::fs::File;
use std::io::{Read, Write};
use std::collections::BTreeMap;

pub use errors::*;
use objects::*;

pub mod objects;

mod errors {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
            Yaml(::serde_yaml::Error);
            Json(::serde_json::Error);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybeRef<T> {
    Concrete(T),
    Ref(Ref),
}

type Map<T> = BTreeMap<String, T>;
type MapMaybeRef<T> = Map<MaybeRef<T>>;

impl<T> MaybeRef<T> {
    pub fn resolve_ref<'a>(&'a self, map: &'a MapMaybeRef<T>) -> Result<&'a T> {
        match *self {
            MaybeRef::Concrete(ref inner) => Ok(inner),
            MaybeRef::Ref(ref r) => {
                match r.ref_.rfind("/") {
                    None => bail!("Reference {} is not valid path", r.ref_),
                    Some(loc) => {
                        let (_, name) = r.ref_.split_at(loc + 1);
                        match map.get(name) {
                            Some(&MaybeRef::Concrete(ref inner)) => Ok(inner),
                            Some(&MaybeRef::Ref(ref ref_)) => {
                                bail!("Recursive reference {}", ref_.ref_)
                            }
                            None => bail!("Reference {} not found", name),
                        }
                    }
                }
            }
        }
    }

    pub fn as_result(&self) -> Result<&T> {
        match *self {
            MaybeRef::Concrete(ref t) => Ok(t),
            _ => bail!("MaybeRef not concrete"),
        }
    }

    pub fn resolve_ref_opt<'a>(&'a self, maybe_map: &'a Option<MapMaybeRef<T>>) -> Result<&'a T> {
        match *maybe_map {
            Some(ref map) => self.resolve_ref(map),
            None => self.as_result(),
        }
    }
}

/// The root struct representing an OpenAPI spec
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpenApi {
    pub openapi: String,
    pub info: Info,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<Server>>,
    pub paths: Map<Path>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Components>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<SecurityRequirement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tag>,
    #[serde(rename = "externalDocs")]
    #[serde(skip_serializing_if = "Option::is_none")]
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

    pub fn from_string(s: &str) -> Result<OpenApi> {
        OpenApi::from_reader(s.as_bytes())
    }

    pub fn to_yaml<W: Write>(&self, writer: W) -> Result<()> {
        Ok(serde_yaml::to_writer(writer, &self)?)
    }

    pub fn to_yaml_string(&self) -> Result<String> {
        Ok(serde_yaml::to_string(&self)?)
    }

    pub fn to_json<W: Write>(&self, writer: W) -> Result<()> {
        Ok(serde_json::to_writer(writer, &self)?)
    }

    pub fn to_json_string(&self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Test serialization

    #[test]
    fn parse_petstore() {
        let s = include_str!("../test_specs/petstore.yaml");
        let api = OpenApi::from_string(s).unwrap();
        println!("{:#?}", api)
    }

    #[test]
    fn parse_and_serialize_petstore() {
        let s = include_str!("../test_specs/petstore.yaml");
        let api = OpenApi::from_string(s).unwrap();
        let yaml = api.to_yaml_string().unwrap();
        println!("{}", yaml);
    }

    #[test]
    fn parse_petstore_expanded() {
        let s = include_str!("../test_specs/petstore-expanded.yaml");
        let api = OpenApi::from_string(s).unwrap();
        println!("{:#?}", api)
    }

    #[test]
    fn parse_uber() {
        let s = include_str!("../test_specs/uber.yaml");
        let api = OpenApi::from_string(s).unwrap();
        println!("{:#?}", api)
    }
}
