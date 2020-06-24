use std::str::FromStr;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::errors;

mod json;
mod toml;
mod yaml;

#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter)]
pub enum Format {
    Json,
    Toml,
    Yaml,
}

impl Format {
    pub fn names() -> Vec<&'static str> {
        Format::iter().map(|f| f.name()).collect()
    }

    pub fn name(&self) -> &'static str {
        match *self {
            Format::Json => "json",
            Format::Toml => "toml",
            Format::Yaml => "yaml",
        }
    }

    pub fn extensions(&self) -> &[&'static str] {
        match *self {
            Format::Json => &["json"],
            Format::Toml => &["toml"],
            Format::Yaml => &["yaml", "yml"],
        }
    }

    pub fn is_extension(&self, s: &str) -> bool {
        self.extensions().iter().find(|&&ext| ext == s).is_some()
    }

    pub fn preferred_extension(&self) -> &'static str {
        self.name()
    }
}

impl FromStr for Format {
    type Err = errors::Error;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let lower = s.to_ascii_lowercase();
        Ok(Format::iter()
            .find(|f| f.is_extension(&lower))
            .ok_or(errors::Error::FormatName(s.to_string()))?)
    }
}

pub struct FormattedText {
    pub format: Format,
    pub text: String,
}

impl FormattedText {
    pub fn new(format: Format, text: String) -> FormattedText {
        FormattedText { format, text }
    }

    pub fn convert_to(&self, format: Format) -> Result<FormattedText, errors::Error> {
        match format {
            Format::Json => self.to_json(),
            Format::Toml => self.to_toml(),
            Format::Yaml => self.to_yaml(),
        }
        .map(|text| FormattedText { text, format })
    }

    fn to_json(&self) -> Result<String, errors::Error> {
        let value = self.deserialize::<json::InnerValue>(&self.text)?;
        json::serialize(&value)
    }

    fn to_toml(&self) -> Result<String, errors::Error> {
        let value = self.deserialize::<toml::InnerValue>(&self.text)?;
        toml::serialize(&value)
    }
    fn to_yaml(&self) -> Result<String, errors::Error> {
        let value = self.deserialize::<yaml::InnerValue>(&self.text)?;
        yaml::serialize(&value)
    }

    fn deserialize<V>(&self, s: &str) -> Result<V, errors::Error>
    where
        V: for<'de> serde::Deserialize<'de>,
    {
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

    static YAML_TEXT: &'static str = r#"---
id: 123
title: Lorem ipsum dolor sit amet
author:
  id: 999
  first_name: John
  last_name: Doe"#;

    #[test]
    fn format_from_str() {
        assert_eq!(Format::Json, Format::from_str("json").unwrap());
        assert_eq!(Format::Json, Format::from_str("JsOn").unwrap());

        assert_eq!(Format::Toml, Format::from_str("toml").unwrap());
        assert_eq!(Format::Yaml, Format::from_str("yaml").unwrap());
        assert_eq!(Format::Yaml, Format::from_str("yml").unwrap());

        let r = Format::from_str("conf"); // HOCON
        assert!(r.is_err());
    }

    #[test]
    fn convert_json() {
        let text = FormattedText::new(Format::Json, JSON_TEXT.to_string());

        // JSON => TOML
        let r = text.convert_to(Format::Toml);
        assert!(r.is_ok());
        assert_eq!(
            r#"id = 123
title = "Lorem ipsum dolor sit amet"

[author]
first_name = "John"
id = 999
last_name = "Doe"
"#,
            r.as_ref().ok().unwrap().text
        );

        // JSON => YAML
        let r = text.convert_to(Format::Yaml);
        assert!(r.is_ok());
        assert_eq!(YAML_TEXT, r.as_ref().ok().unwrap().text);

        // Error
        let text = FormattedText::new(Format::Json, YAML_TEXT.to_string());
        let r = text.convert_to(Format::Toml);
        assert!(r.is_err());
    }

    #[test]
    fn convert_toml() {
        let text = FormattedText::new(
            Format::Toml,
            r#"id = 123
title = "Lorem ipsum dolor sit amet"

[author]
id = 999
first_name = "John"
last_name = "Doe"
"#
            .to_string(),
        );

        // TOML => JSON
        let r = text.convert_to(Format::Json);
        assert!(r.is_ok());
        assert_eq!(JSON_TEXT, r.as_ref().ok().unwrap().text);

        // TOML => YAML
        let r = text.convert_to(Format::Yaml);
        assert!(r.is_ok());
        assert_eq!(YAML_TEXT, r.as_ref().ok().unwrap().text);

        // Error
        let text = FormattedText::new(Format::Toml, JSON_TEXT.to_string());
        let r = text.convert_to(Format::Yaml);
        assert!(r.is_err());
    }

    #[test]
    fn convert_yaml() {
        let text = FormattedText::new(Format::Yaml, YAML_TEXT.to_string());

        // YAML => JSON
        let r = text.convert_to(Format::Json);
        assert!(r.is_ok());
        assert_eq!(JSON_TEXT, r.as_ref().ok().unwrap().text);

        // YAML => TOML
        let r = text.convert_to(Format::Toml);
        assert!(r.is_ok());
        assert_eq!(
            r#"id = 123
title = "Lorem ipsum dolor sit amet"

[author]
first_name = "John"
id = 999
last_name = "Doe"
"#,
            r.as_ref().ok().unwrap().text
        );

        // Error
        // TODO: this test will be panicked on `r.is_err()`. need to survey why the panic occurs..
        //        let text = FormattedText::new(Format::Yaml, TOML_TEXT.to_string());
        //        let r = text.convert_to(Format::Json);
        //        assert!(r.is_err());
        //        assert_eq!(errors::ErrorKind::Deserialization, r.err().unwrap().kind());
    }
}
