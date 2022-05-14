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
    pub name: String,
    pub version: String,
    pub dependencies: Option<HashMap<String, String>>,
    pub dist: Dist,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct Dist {
    pub tarball: String,
    pub shasum: String,
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
}
