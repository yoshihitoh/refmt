use std::io;

use refmt_serde::RefmtError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO Error. cause:{_0}")]
    Io(#[from] io::Error),

    #[error("Any errors occurred on re-format. cause:{_0}")]
    RefmtError(#[from] RefmtError),

    #[error("Unsupported format name. name:{_0}")]
    FormatName(String),

    #[error("Cannot infer format. Please specify either FILE or FORMAT")]
    InferFormat,

    #[error("Cannot create assets")]
    CreatingAssets(String),
}
