use std::collections::HashMap;

use async_recursion::async_recursion;
use lazy_static::lazy_static;
use node_semver::{Range, Version};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use std::fmt::Debug;

#[derive(PartialEq, Debug)]
pub enum Error {
    PackageNotFound,
    NetworkError,
    InvalidResponse,
    RangeNotSatisfied,
    InvalidRange,
}

lazy_static! {
    static ref CLIENT: Client = Client::new();
}

#[derive(Deserialize)]
struct RawMetadata {
    versions: HashMap<Version, Metadata>,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct Metadata {
    name: String,
    version: String,
    dependencies: Option<HashMap<String, String>>,
    dist: Dist,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct Dist {
    tarball: String,
    shasum: String,
}

pub async fn get_metadata(name: String, range: Range, registry: &str) -> Result<Metadata, Error> {
    let response = match CLIENT.get(format!("{registry}/{name}")).send().await {
        Ok(response) => response,
        Err(e) => {
            if e.status() == Some(StatusCode::NOT_FOUND) {
                return Err(Error::PackageNotFound);
            } else {
                return Err(Error::NetworkError);
            }
        }
    }
    .json::<RawMetadata>()
    .await;

    let response = match response {
        Ok(r) => r,
        _ => return Err(Error::InvalidResponse),
    };

    for (key, value) in response.versions {
        if range.satisfies(&key) {
            return Ok(value);
        }
    }

    Err(Error::RangeNotSatisfied)
}

/// **note:** this function does NOT dedup the result
/// (users should merge together the resulting Vecs and dedup then)
#[async_recursion]
pub async fn walk_dependencies(
    name: &String,
    range: Range,
    registry: &str,
) -> Result<Vec<Metadata>, Error> {
    let mut result: Vec<Metadata> = Vec::with_capacity(256);

    let metadata = get_metadata(name.to_string(), range, registry).await?;
    let deps = metadata.dependencies.clone();
    result.push(metadata);

    if let Some(deps) = deps {
        for (key, value) in deps {
            let range = match Range::parse(value) {
                Ok(r) => r,
                Err(_) => return Err(Error::InvalidRange),
            };
            let mut walked = walk_dependencies(&key, range, registry).await?;
            result.append(&mut walked);
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common_macros::hash_map;

    const REGISTRY: &str = "https://registry.npmjs.org";

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    macro_rules! panic_on_err {
        ($e:expr) => {
            match $e {
                Ok(r) => r,
                _ => panic!("error! error!"),
            }
        };
    }

    macro_rules! s {
        ($e:expr) => {
            String::from($e)
        };
    }

    #[test]
    fn valid_lodash() {
        let metadata = aw!(get_metadata(
            s!("lodash"),
            Range::parse("1.2.1").unwrap(),
            REGISTRY
        ));
        assert_eq!(
            panic_on_err!(metadata),
            Metadata {
                name: s!("lodash"),
                version: s!("1.2.1"),
                dependencies: None,
                dist: Dist {
                    // Tell DevSkim that a SHA-1 hash, funnily enough, is not a token
                    // DevSkim: ignore DS173237
                    shasum: s!("ed47b16e46f06b2b40309b68e9163c17e93ea304"),
                    tarball: s!("https://registry.npmjs.org/lodash/-/lodash-1.2.1.tgz")
                }
            }
        );
    }

    #[test]
    fn valid_inflight() {
        let walked = aw!(walk_dependencies(
            &s!("inflight"),
            Range::parse("1.0.6").unwrap(),
            REGISTRY
        ));
        assert_eq!(
            panic_on_err!(walked),
            vec![
                Metadata {
                    name: s!("inflight"),
                    version: s!("1.0.6"),
                    dependencies: Some(hash_map! {
                        s!("wrappy") => s!("1"),
                        s!("once") => s!("^1.3.0")
                    }),
                    dist: Dist {
                        tarball: s!("https://registry.npmjs.org/inflight/-/inflight-1.0.6.tgz"),
                        shasum: s!("49bd6331d7d02d0c09bc910a1075ba8165b56df9")
                    }
                },
                Metadata {
                    name: s!("wrappy"),
                    version: s!("1.0.2"),
                    dependencies: Some(HashMap::new()),
                    dist: Dist {
                        tarball: s!("https://registry.npmjs.org/wrappy/-/wrappy-1.0.2.tgz"),
                        shasum: s!("b5243d8f3ec1aa35f1364605bc0d1036e30ab69f")
                    }
                },
                Metadata {
                    name: s!("once"),
                    version: s!("1.3.3"),
                    dependencies: Some(hash_map! {s!("wrappy") => s!("1")}),
                    dist: Dist {
                        tarball: s!("https://registry.npmjs.org/once/-/once-1.3.3.tgz"),
                        shasum: s!("b2e261557ce4c314ec8304f3fa82663e4297ca20")
                    }
                },
                Metadata {
                    name: s!("wrappy"),
                    version: s!("1.0.1"),
                    dependencies: Some(HashMap::new()),
                    dist: Dist {
                        tarball: s!("https://registry.npmjs.org/wrappy/-/wrappy-1.0.1.tgz"),
                        shasum: s!("1e65969965ccbc2db4548c6b84a6f2c5aedd4739")
                    }
                }
            ]
        )
    }
}
