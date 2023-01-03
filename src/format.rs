use std::str::FromStr;

use refmt_serde::{Format, Refmt};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::errors;

#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter)]
pub enum FileFormat {
    Json,
    Toml,
    Yaml,
}

impl FileFormat {
    pub fn names() -> Vec<&'static str> {
        FileFormat::iter().map(|f| f.name()).collect()
    }

    pub fn name(&self) -> &'static str {
        match *self {
            FileFormat::Json => "json",
            FileFormat::Toml => "toml",
            FileFormat::Yaml => "yaml",
        }
    }

    pub fn extensions(&self) -> &[&'static str] {
        match *self {
            FileFormat::Json => &["json"],
            FileFormat::Toml => &["toml"],
            FileFormat::Yaml => &["yaml", "yml"],
        }
    }

    pub fn is_extension(&self, s: &str) -> bool {
        self.extensions().iter().find(|&&ext| ext == s).is_some()
    }

    pub fn preferred_extension(&self) -> &'static str {
        self.name()
    }
}

impl FromStr for FileFormat {
    type Err = errors::Error;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let lower = s.to_ascii_lowercase();
        Ok(FileFormat::iter()
            .find(|f| f.is_extension(&lower))
            .ok_or(errors::Error::FormatName(s.to_string()))?)
    }
}

impl From<FileFormat> for Format {
    fn from(value: FileFormat) -> Self {
        match value {
            FileFormat::Json => Format::Json,
            FileFormat::Yaml => Format::Yaml,
            FileFormat::Toml => Format::Toml,
        }
    }
}

pub struct FormattedText {
    pub format: FileFormat,
    pub text: String,
}

impl FormattedText {
    pub fn new(format: FileFormat, text: String) -> FormattedText {
        FormattedText { format, text }
    }

    pub fn convert_to(&self, format: FileFormat) -> Result<FormattedText, errors::Error> {
        let refmt = Refmt {
            src_format: Format::from(self.format),
            dest_format: Format::from(format),
        };

        let text = refmt.refmt(&self.text)?;
        Ok(FormattedText { text, format })
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
}
"#;

    static YAML_TEXT: &'static str = r#"id: 123
title: Lorem ipsum dolor sit amet
author:
  id: 999
  first_name: John
  last_name: Doe
"#;

    #[test]
    fn format_from_str() {
        assert_eq!(FileFormat::Json, FileFormat::from_str("json").unwrap());
        assert_eq!(FileFormat::Json, FileFormat::from_str("JsOn").unwrap());

        assert_eq!(FileFormat::Toml, FileFormat::from_str("toml").unwrap());
        assert_eq!(FileFormat::Yaml, FileFormat::from_str("yaml").unwrap());
        assert_eq!(FileFormat::Yaml, FileFormat::from_str("yml").unwrap());

        let r = FileFormat::from_str("conf"); // HOCON
        assert!(r.is_err());
    }

    #[test]
    fn convert_json() {
        let text = FormattedText::new(FileFormat::Json, JSON_TEXT.to_string());

        // JSON => TOML
        let r = text.convert_to(FileFormat::Toml);
        assert!(r.is_ok());
        assert_eq!(
            r#"id = 123
title = "Lorem ipsum dolor sit amet"

[author]
id = 999
first_name = "John"
last_name = "Doe"
"#,
            r.as_ref().ok().unwrap().text
        );

        // JSON => YAML
        let r = text.convert_to(FileFormat::Yaml);
        assert!(r.is_ok());
        assert_eq!(YAML_TEXT, r.as_ref().ok().unwrap().text);

        // Error
        let text = FormattedText::new(FileFormat::Json, YAML_TEXT.to_string());
        let r = text.convert_to(FileFormat::Toml);
        assert!(r.is_err());
    }

    #[test]
    fn convert_toml() {
        let text = FormattedText::new(
            FileFormat::Toml,
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
        let r = text.convert_to(FileFormat::Json);
        assert!(r.is_ok());
        assert_eq!(JSON_TEXT, r.as_ref().ok().unwrap().text);

        // TOML => YAML
        let r = text.convert_to(FileFormat::Yaml);
        assert!(r.is_ok());
        assert_eq!(YAML_TEXT, r.as_ref().ok().unwrap().text);

        // Error
        let text = FormattedText::new(FileFormat::Toml, JSON_TEXT.to_string());
        let r = text.convert_to(FileFormat::Yaml);
        assert!(r.is_err());
    }

    #[test]
    fn convert_yaml() {
        let text = FormattedText::new(FileFormat::Yaml, YAML_TEXT.to_string());

        // YAML => JSON
        let r = text.convert_to(FileFormat::Json);
        assert!(r.is_ok());
        assert_eq!(JSON_TEXT, r.as_ref().ok().unwrap().text);

        // YAML => TOML
        let r = text.convert_to(FileFormat::Toml);
        assert!(r.is_ok());
        assert_eq!(
            r#"id = 123
title = "Lorem ipsum dolor sit amet"

[author]
id = 999
first_name = "John"
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
