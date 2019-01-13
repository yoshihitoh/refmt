use std::fmt::{self, Display, Formatter};

use failure::{Backtrace, Context, Fail};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Fail)]
pub enum ErrorKind {
    #[fail(display = "IO Error")]
    Io,

    #[fail(display = "Any errors occurred during serialization")]
    Serialization,

    #[fail(display = "Any errors occurred during deserialization")]
    Deserialization,

    #[fail(display = "Unsupported format name")]
    FormatName,

    #[fail(display = "Cannot infer format. Please specify either FILE or FORMAT")]
    InferFormat,

    #[fail(display = "Cannot create assets")]
    CreatingAssets,
}

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        *self.inner.get_context()
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(context: Context<ErrorKind>) -> Self {
        Error { inner: context }
    }
}
