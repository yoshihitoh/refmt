use std::str::FromStr;

use serde::{de, ser};
use strum_macros::EnumIter;

use super::json;
use super::toml;
use super::yaml;
use crate::errors;

// NOTE: Use serde_json::Value as intermediate type, it keeps field orders, and have enough type to convert to another format.
type Value = serde_json::Value;

#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter)]
pub enum Format {
    Json,
    Toml,
    Yaml,
}

pub struct FormattedText {
    format: Format,
    text: String,
}

impl FormattedText {
    pub fn new(format: Format, text: String) -> FormattedText {
        FormattedText { format, text }
    }

    pub fn convert_to(&self, format: Format) -> Result<String, errors::Error> {
        let value = self.deserialize(&self.text)?;
        Self::serialize(format, &value)
    }

    fn serialize(format: Format, v: &Value) -> Result<String, errors::Error> {
        match format {
            Format::Json => json::serialize(v),
            Format::Toml => toml::serialize(v),
            Format::Yaml => yaml::serialize(v),
        }
    }

    fn deserialize(&self, s: &str) -> Result<Value, errors::Error> {
        match self.format {
            Format::Json => json::deserialize(s),
            Format::Toml => toml::deserialize(s),
            Format::Yaml => yaml::deserialize(s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static JSON_TEXT: &'static str = r#"{
  "id": 123,
  "title": "Lorem ipsum dolor sit amet",
  "author": {
    "id": 999,
    "first_name": "John",
    "last_name": "Doe"
  }
}"#;

    static TOML_TEXT: &'static str = r#"id = 123
title = "Lorem ipsum dolor sit amet"

[author]
id = 999
first_name = "John"
last_name = "Doe"
"#;

    static YAML_TEXT: &'static str = r#"---
id: 123
title: Lorem ipsum dolor sit amet
author:
  id: 999
  first_name: John
  last_name: Doe"#;

    #[test]
    fn convert_json() {
        let text = FormattedText::new(Format::Json, JSON_TEXT.to_string());

        // JSON => TOML
        let r = text.convert_to(Format::Toml);
        assert!(r.is_ok());
        assert_eq!(TOML_TEXT, r.unwrap());

        // JSON => YAML
        let r = text.convert_to(Format::Yaml);
        assert!(r.is_ok());
        assert_eq!(YAML_TEXT, r.unwrap());
    }

    #[test]
    fn convert_toml() {
        let text = FormattedText::new(Format::Toml, TOML_TEXT.to_string());

        // TOML => JSON
        let r = text.convert_to(Format::Json);
        assert!(r.is_ok());
        assert_eq!(JSON_TEXT, r.unwrap());

        // TOML => YAML
        let r = text.convert_to(Format::Yaml);
        assert!(r.is_ok());
        assert_eq!(YAML_TEXT, r.unwrap());
    }

    #[test]
    fn convert_yaml() {
        let text = FormattedText::new(Format::Yaml, YAML_TEXT.to_string());

        // YAML => JSON
        let r = text.convert_to(Format::Json);
        assert!(r.is_ok());
        assert_eq!(JSON_TEXT, r.unwrap());

        // YAML => TOML
        let r = text.convert_to(Format::Toml);
        assert!(r.is_ok());
        assert_eq!(TOML_TEXT, r.unwrap());
    }
}
