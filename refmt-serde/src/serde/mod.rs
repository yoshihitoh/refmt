use serde::{Deserialize, Serialize};

pub mod json;
pub mod toml;
pub mod yaml;

pub trait Serde {
    type Error: std::error::Error;
    type ValueType: Serialize;

    fn serialize(&self, v: &Self::ValueType) -> Result<String, Self::Error>;
    fn deserialize<T: for<'de> Deserialize<'de>>(&self, s: &str) -> Result<T, Self::Error>;
}
