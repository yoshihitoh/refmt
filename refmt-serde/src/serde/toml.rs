use serde::Deserialize;

use super::Serde;

#[derive(Debug, thiserror::Error)]
pub enum TomlError {
    #[error("can't serialize into toml.")]
    Serialize(#[from] toml::ser::Error),

    #[error("can't deserialize from toml.")]
    Deserialize(#[from] toml::de::Error),
}

pub struct Toml;

impl Serde for Toml {
    type Error = TomlError;
    type ValueType = toml::Value;

    fn serialize(&self, v: &Self::ValueType) -> Result<String, Self::Error> {
        let s = toml::to_string(v)?;
        Ok(s)
    }

    fn deserialize<T: for<'de> Deserialize<'de>>(&self, s: &str) -> Result<T, Self::Error> {
        let v = toml::from_str(s)?;
        Ok(v)
    }
}
