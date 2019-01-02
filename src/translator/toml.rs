use serde::Deserialize;

use super::translator::{parse_as, to_translate_error, Format, TranslateError, Translator};

pub struct TomlTranslator {}

impl Default for TomlTranslator {
    fn default() -> Self {
        TomlTranslator {}
    }
}

impl Translator for TomlTranslator {
    fn translate(&self, s: &str, fmt: Format) -> Result<String, TranslateError> {
        let value = parse_as::<toml::Value>(s, fmt)?;
        toml::to_string_pretty(&value).map_err(to_translate_error)
    }
}

pub fn parse_toml<'de, V: Deserialize<'de>>(s: &'de str) -> Result<V, toml::de::Error> {
    toml::from_str(s)
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
    fn test_translate() {
        let translator = TomlTranslator {};
        let json_text = r#"
        {
            "id": 1,
            "name": {
                "first": "John",
                "last": "Doe"
            },
            "repos": [
                { "name": "zstd-codec", "language": "C++"},
                { "name": "reser", "language": "Rust"}
            ]
        }
        "#;
        let toml = translator.translate(json_text, Format::Json);
        assert!(toml.is_ok());
        let toml = toml.ok().unwrap();

        // REF: https://github.com/alexcrichton/toml-rs/issues/232
        // toml-rs 0.4 doesn't support `preserve_order` feature, and using `BTreeMap` internally.
        // So we need to compare items in alphabetical order.
        // The `preserve_order` feature is planned to release on toml-rs 0.5.
        assert_eq!(
            &toml,
            r#"id = 1

[[repos]]
language = 'C++'
name = 'zstd-codec'

[[repos]]
language = 'Rust'
name = 'reser'

[name]
first = 'John'
last = 'Doe'
"#
        );
    }

    #[test]
    fn test_parse_toml() {
        let toml_text = r#"
[package]
name = "reser"
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

        let manifest = parse_toml::<Manifest>(toml_text);
        assert!(manifest.is_ok());
        let manifest = manifest.ok().unwrap();
        assert_eq!(manifest.package.name, "reser");
        assert_eq!(
            manifest.package.authors,
            vec!["yoshihitoh <yoshihito.arih@gmail.com>"]
        );
        assert_eq!(manifest.package.edition, Some("2018".to_string()));

        assert!(manifest.dependencies.is_some());
        let dependencies = manifest.dependencies.unwrap();
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
    }
}
