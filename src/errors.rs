use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO Error. cause:{_0}")]
    Io(#[from] io::Error),

    #[error("Any errors occurred during serialization")]
    Serialization(String),

    #[error("Any errors occurred during deserialization")]
    Deserialization(String),

    #[error("Unsupported format name. name:{_0}")]
    FormatName(String),

    #[error("Cannot infer format. Please specify either FILE or FORMAT")]
    InferFormat,

    #[error("Cannot create assets")]
    CreatingAssets(String),
}
