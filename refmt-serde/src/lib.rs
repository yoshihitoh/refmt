use ::serde::Deserialize;

use crate::serde::{json, toml, yaml, Serde};

pub mod serde;

#[derive(Debug, Copy, Clone)]
pub enum Format {
    Json,
    Toml,
    Yaml,
}

#[derive(Debug, thiserror::Error)]
pub enum RefmtError {
    #[error("json error.")]
    Json(#[from] <json::Json as Serde>::Error),

    #[error("toml error.")]
    Toml(#[from] <toml::Toml as Serde>::Error),

    #[error("yaml error.")]
    Yaml(#[from] <yaml::Yaml as Serde>::Error),
}

pub struct Refmt {
    pub src_format: Format,
    pub dest_format: Format,
}

impl Refmt {
    pub fn refmt(&self, s: &str) -> Result<String, RefmtError> {
        let r = match self.dest_format {
            Format::Json => self.to_json(s)?,
            Format::Toml => self.to_toml(s)?,
            Format::Yaml => self.to_yaml(s)?,
        };
        Ok(r)
    }

    fn to_json(&self, s: &str) -> Result<String, RefmtError> {
        let v = self.deserialize::<<json::Json as Serde>::ValueType>(s)?;
        let s = json::Json.serialize(&v)?;
        Ok(s)
    }

    fn to_toml(&self, s: &str) -> Result<String, RefmtError> {
        let v = self.deserialize::<<toml::Toml as Serde>::ValueType>(s)?;
        let s = toml::Toml.serialize(&v)?;
        Ok(s)
    }

    fn to_yaml(&self, s: &str) -> Result<String, RefmtError> {
        let v = self.deserialize::<<yaml::Yaml as Serde>::ValueType>(s)?;
        let s = yaml::Yaml.serialize(&v)?;
        Ok(s)
    }

    fn deserialize<T>(&self, s: &str) -> Result<T, RefmtError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let r = match self.src_format {
            Format::Json => json::Json.deserialize(s)?,
            Format::Toml => toml::Toml.deserialize(s)?,
            Format::Yaml => yaml::Yaml.deserialize(s)?,
        };
        Ok(r)
    }
}

#[cfg(test)]
mod tests {
    mod fixtures {
        use once_cell::sync::Lazy;
        pub static JSON: Lazy<String> = Lazy::new(|| {
            r#"
{
  "id": 123,
  "title": "Lorem ipsum dolor sit amet",
  "author": {
    "id": 999,
    "first_name": "John",
    "last_name": "Doe"
  }
}
"#
            .trim_start()
            .to_string()
        });

        pub static TOML: Lazy<String> = Lazy::new(|| {
            r#"
id = 123
title = "Lorem ipsum dolor sit amet"

[author]
id = 999
first_name = "John"
last_name = "Doe"
"#
            .trim_start()
            .to_string()
        });

        pub static YAML: Lazy<String> = Lazy::new(|| {
            //
            r#"
id: 123
title: Lorem ipsum dolor sit amet
author:
  id: 999
  first_name: John
  last_name: Doe
"#
            .trim_start()
            .to_string()
        });
    }

    use crate::{Format, Refmt};
    use fixtures::{JSON, TOML, YAML};

    fn refmt(src_format: Format, dest_format: Format) -> Refmt {
        Refmt {
            src_format,
            dest_format,
        }
    }

    #[test]
    fn json_to_json() {
        let refmt = refmt(Format::Json, Format::Json);
        let r = refmt.refmt(&JSON);
        assert_eq!(Some(JSON.to_string()), r.ok());
    }

    #[test]
    fn json_to_toml() {
        let refmt = refmt(Format::Json, Format::Toml);
        let r = refmt.refmt(&JSON);
        assert_eq!(Some(TOML.to_string()), r.ok());
    }

    #[test]
    fn json_to_yaml() {
        let refmt = refmt(Format::Json, Format::Yaml);
        let r = refmt.refmt(&JSON);
        assert_eq!(Some(YAML.to_string()), r.ok());
    }

    #[test]
    fn toml_to_json() {
        let refmt = refmt(Format::Toml, Format::Json);
        let r = refmt.refmt(&TOML);
        assert_eq!(Some(JSON.to_string()), r.ok());
    }

    #[test]
    fn toml_to_toml() {
        let refmt = refmt(Format::Toml, Format::Toml);
        let r = refmt.refmt(&TOML);
        assert_eq!(Some(TOML.to_string()), r.ok());
    }

    #[test]
    fn toml_to_yaml() {
        let refmt = refmt(Format::Toml, Format::Yaml);
        let r = refmt.refmt(&TOML);
        assert_eq!(Some(YAML.to_string()), r.ok());
    }

    #[test]
    fn yaml_to_json() {
        let refmt = refmt(Format::Yaml, Format::Json);
        let r = refmt.refmt(&YAML);
        assert_eq!(Some(JSON.to_string()), r.ok());
    }

    #[test]
    fn yaml_to_toml() {
        let refmt = refmt(Format::Yaml, Format::Toml);
        let r = refmt.refmt(&YAML);
        assert_eq!(Some(TOML.to_string()), r.ok());
    }

    #[test]
    fn yaml_to_yaml() {
        let refmt = refmt(Format::Yaml, Format::Yaml);
        let r = refmt.refmt(&YAML);
        assert_eq!(Some(YAML.to_string()), r.ok());
    }
}
