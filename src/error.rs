use std::error::Error;
use std::fmt;
use std::io;
use std::num::ParseIntError;
use std::string::ParseError;

use crate::cli::error::CliError;
use crate::protocol::error::ProtocolError;

#[derive(Debug)]
pub enum CordErrorKind {
    Io,
    Parse,
    Protocol,
    Cli,
    Other,
}

#[derive(Debug)]
pub struct CordError {
    kind: CordErrorKind,
    source: Box<dyn Error + Send + Sync + 'static>,
}

impl CordError {
    pub fn new<E>(kind: CordErrorKind, error: E) -> Self
    where
        E: Into<Box<dyn Error + Send + Sync + 'static>>,
    {
        Self {
            kind,
            source: error.into(),
        }
    }

    pub fn kind(&self) -> &CordErrorKind {
        &self.kind
    }
}

impl fmt::Display for CordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            CordErrorKind::Io => write!(f, "IO error: {}", self.source),
            CordErrorKind::Parse => write!(f, "Parse error: {}", self.source),
            CordErrorKind::Protocol => write!(f, "Protocol error: {}", self.source),
            CordErrorKind::Cli => write!(f, "Cli error: {}", self.source),
            CordErrorKind::Other => write!(f, "Error: {}", self.source),
        }
    }
}

impl Error for CordError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&*self.source)
    }
}

impl From<io::Error> for CordError {
    fn from(err: io::Error) -> Self {
        Self::new(CordErrorKind::Io, err)
    }
}

impl From<ParseError> for CordError {
    fn from(err: ParseError) -> Self {
        Self::new(CordErrorKind::Parse, err)
    }
}

impl From<ParseIntError> for CordError {
    fn from(err: ParseIntError) -> Self {
        Self::new(CordErrorKind::Parse, err)
    }
}

impl From<ProtocolError> for CordError {
    fn from(err: ProtocolError) -> Self {
        Self::new(CordErrorKind::Protocol, err)
    }
}

impl From<CliError> for CordError {
    fn from(err: CliError) -> Self {
        Self::new(CordErrorKind::Cli, err)
    }
}

impl From<String> for CordError {
    fn from(err: String) -> Self {
        Self::new(
            CordErrorKind::Other,
            io::Error::new(io::ErrorKind::Other, err),
        )
    }
}

impl From<&str> for CordError {
    fn from(err: &str) -> Self {
        Self::from(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, CordError>;
