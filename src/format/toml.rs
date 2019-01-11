use failure::ResultExt;
use serde::de;
use serde::ser;

use crate::errors::{self, ErrorKind};

pub fn serialize<V: ser::Serialize>(v: V) -> Result<String, errors::Error> {
    Ok(toml::to_string(&v).context(ErrorKind::Serialization)?)
}

pub fn deserialize<V>(s: &str) -> Result<V, errors::Error>
where
    V: for<'de> de::Deserialize<'de>,
{
    Ok(toml::from_str(s).context(ErrorKind::Deserialization)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_derive::{Deserialize, Serialize};
    use std::collections::BTreeMap;

    #[derive(Serialize, Deserialize, Debug)]
    struct Package {
        pub name: String,
        pub authors: Vec<String>,
        pub edition: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, PartialOrd, PartialEq)]
    #[serde(untagged)]
    enum Dependency {
        Simple(String),
        Detailed(DetailedDependency),
    }

    #[derive(Serialize, Deserialize, Debug, PartialOrd, PartialEq)]
    struct DetailedDependency {
        pub version: String,
        pub features: Option<Vec<String>>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Manifest {
        pub package: Package,
        pub dependencies: Option<BTreeMap<String, Dependency>>,
    }

    #[test]
    fn success() {
        let toml_text = r#"
[package]
name = "refmt"
authors = ["yoshihitoh <yoshihito.arih@gmail.com>"]
edition = "2018"

[dependencies]
clap = "2.32"
serde = "1.0"
serde_json = { version = "1.0", features = ["preserve_order"] }
serde_yaml = "0.8"
strum = "0.13"
strum_macros = "0.13"
toml = "0.4"
"#;

        // deserialize
        let manifest = deserialize::<Manifest>(toml_text);
        assert!(manifest.is_ok());
        let manifest = manifest.ok().unwrap();
        assert_eq!(manifest.package.name, "refmt");
        assert_eq!(
            manifest.package.authors,
            vec!["yoshihitoh <yoshihito.arih@gmail.com>"]
        );
        assert_eq!(manifest.package.edition, Some("2018".to_string()));

        assert!(manifest.dependencies.is_some());
        let dependencies = manifest.dependencies.as_ref().unwrap();
        assert_eq!(dependencies.len(), 7);
        assert_eq!(
            dependencies.get("clap"),
            Some(&Dependency::Simple("2.32".to_string()))
        );
        assert_eq!(
            dependencies.get("serde"),
            Some(&Dependency::Simple("1.0".to_string()))
        );
        assert_eq!(
            dependencies.get("serde_json"),
            Some(&Dependency::Detailed(DetailedDependency {
                version: "1.0".to_string(),
                features: Some(vec!["preserve_order".to_string()]),
            }))
        );
        assert_eq!(
            dependencies.get("serde_yaml"),
            Some(&Dependency::Simple("0.8".to_string()))
        );
        assert_eq!(
            dependencies.get("strum"),
            Some(&Dependency::Simple("0.13".to_string()))
        );
        assert_eq!(
            dependencies.get("strum_macros"),
            Some(&Dependency::Simple("0.13".to_string()))
        );
        assert_eq!(
            dependencies.get("toml"),
            Some(&Dependency::Simple("0.4".to_string()))
        );

        // serialize
        // TODO: cannot serialize manifest object due to `ValueAfterTable` error. I need to survey why the error occurs.
        let toml_text = serialize(&manifest.package);
        assert!(toml_text.is_ok());
        let toml_text = toml_text.ok().unwrap();

        // NOTE: toml-rs uses BTreeMap internally, so the result will be alphabetical ordered.
        assert_eq!(
            r#"name = "refmt"
authors = ["yoshihitoh <yoshihito.arih@gmail.com>"]
edition = "2018"
"#,
            toml_text
        );
    }
}
