use super::Serde;
use serde::Deserialize;

pub struct Yaml;

impl Serde for Yaml {
    type Error = serde_yaml::Error;
    type ValueType = serde_yaml::Value;

    fn serialize(&self, v: &Self::ValueType) -> Result<String, Self::Error> {
        let s = serde_yaml::to_string(v)?;
        Ok(s)
    }

    fn deserialize<T: for<'de> Deserialize<'de>>(&self, s: &str) -> Result<T, Self::Error> {
        let v = serde_yaml::from_str(s)?;
        Ok(v)
    }
}
