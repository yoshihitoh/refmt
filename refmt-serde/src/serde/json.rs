use super::Serde;
use serde::Deserialize;

pub struct Json;

impl Serde for Json {
    type Error = serde_json::Error;
    type ValueType = serde_json::Value;

    fn serialize(&self, v: &Self::ValueType) -> Result<String, Self::Error> {
        let mut s = serde_json::to_string_pretty(v)?;
        s.push('\n'); // add a new-line for consistency. (YAML and TOML have a new-line on its tail.)
        Ok(s)
    }

    fn deserialize<T: for<'de> Deserialize<'de>>(&self, s: &str) -> Result<T, Self::Error> {
        let v = serde_json::from_str(s)?;
        Ok(v)
    }
}
