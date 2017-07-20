#[macro_use]
extern crate error_chain;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate regex;
#[macro_use]
extern crate derive_new;

use std::fs::File;
use std::io::Read;
use std::collections::BTreeMap;

pub use errors::*;
use objects::*;
pub use process::Entrypoint;

pub mod objects;
pub mod process;

mod errors {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
            Yaml(::serde_yaml::Error);
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
    pub fn resolve_ref<'a>(&'a self, map: &'a MapMaybeRef<T>) -> Option<&'a T> {
        match *self {
            MaybeRef::Concrete(ref inner) => Some(inner),
            MaybeRef::Ref(ref r) => {
                match r.ref_.rfind("/") {
                    None => None,
                    Some(loc) => {
                        let (_, name) = r.ref_.split_at(loc);
                        match map.get(name) {
                            Some(&MaybeRef::Concrete(ref inner)) => Some(inner),
                            _ => None,
                        }
                    }
                }
            }
        }
    }

    pub fn as_option(&self) -> Option<&T> {
        match *self {
            MaybeRef::Concrete(ref t) => Some(t),
            _ => None
        }
    }
}

/// The root struct representing an OpenAPI spec
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpenApi {
    pub openapi: String,
    pub info: Info,
    pub servers: Option<Vec<Server>>,
    pub paths: Map<Path>,
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
        let api = match OpenApi::from_reader(file) {
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
