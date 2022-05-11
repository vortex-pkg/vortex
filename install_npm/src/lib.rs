use std::collections::HashMap;

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
}

lazy_static! {
    static ref CLIENT: Client = Client::new();
}

#[derive(Deserialize)]
struct RawMetadata {
    versions: HashMap<Version, Metadata>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Metadata {
    name: String,
    version: String,
    dependencies: Option<HashMap<String, Range>>,
    dist: Dist,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Dist {
    tarball: String,
    shasum: String,
}

pub async fn get_metadata(name: &str, range: Range, registry: &str) -> Result<Metadata, Error> {
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
                _ => panic!("error! error!")
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
            "lodash",
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
                    shasum: s!("ed47b16e46f06b2b40309b68e9163c17e93ea304"),
                    tarball: s!("https://registry.npmjs.org/lodash/-/lodash-1.2.1.tgz")
                }
            }
        );
    }
}
