use std::error::Error;
use std::str::FromStr;

use serde::de::Deserialize;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::json::parse_json;
use super::toml::parse_toml;
use super::yaml::parse_yaml;

#[derive(Copy, Clone, Debug, PartialEq, EnumIter)]
pub enum Format {
    Json,
    Toml,
    Yaml,
}

impl Format {
    pub fn name(&self) -> &'static str {
        match *self {
            Format::Json => "json",
            Format::Toml => "toml",
            Format::Yaml => "yaml",
        }
    }

    pub fn names() -> Vec<&'static str> {
        Format::iter().map(|x| x.name()).collect()
    }
}

impl FromStr for Format {
    type Err = TranslateError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s.to_ascii_lowercase().as_ref() {
            "json" => Some(Format::Json),
            "toml" => Some(Format::Toml),
            "yaml" => Some(Format::Yaml),
            "yml" => Some(Format::Yaml),
            _ => None,
        }
        .ok_or(TranslateError::Message(format!(
            "unsupported format: {}",
            s
        )))
    }
}

#[derive(Debug)]
pub enum TranslateError {
    Message(String),
}

pub trait Translator {
    fn translate(&self, s: &str, fmt: Format) -> Result<String, TranslateError>;
}

pub fn parse_as<V>(s: &str, fmt: Format) -> Result<V, TranslateError>
where
    V: for<'de> Deserialize<'de>,
{
    match fmt {
        Format::Json => parse_json(s).map_err(to_translate_error),
        Format::Toml => parse_toml(s).map_err(to_translate_error),
        Format::Yaml => parse_yaml(s).map_err(to_translate_error),
    }
}

pub fn to_translate_error<E: Error>(e: E) -> TranslateError {
    TranslateError::Message(format!("{}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_name() {
        assert_eq!(Format::Json.name(), "json");
        assert_eq!(Format::Toml.name(), "toml");
        assert_eq!(Format::Yaml.name(), "yaml");
    }

    #[test]
    fn format_from_str() {
        let r = Format::from_str("json");
        assert!(r.is_ok());
        assert_eq!(r.ok().unwrap(), Format::Json);

        let r = Format::from_str("toml");
        assert!(r.is_ok());
        assert_eq!(r.ok().unwrap(), Format::Toml);

        let r = Format::from_str("yaml");
        assert!(r.is_ok());
        assert_eq!(r.ok().unwrap(), Format::Yaml);

        let r = Format::from_str("yml");
        assert!(r.is_ok());
        assert_eq!(r.ok().unwrap(), Format::Yaml);
    }
}
