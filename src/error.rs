use std::error::Error;
use std::fmt;
use std::io;
use std::num::ParseIntError;
use std::string::ParseError;

use crate::cli::error::CliError;
use crate::protocol::error::ProtocolError;

/// Enumerates error types: IO, parsing, protocol, CLI, and others.
#[derive(Debug)]
pub enum CordErrorKind {
    /// Errors related to input/output operations.
    Io,
    /// Errors related to parsing.
    Parse,
    /// Errors related to protocol operations.
    Protocol,
    /// Errors related to CLI operations.
    Cli,
    /// Other unspecified errors.
    Other,
}

/// Represents detailed application errors.
///
/// The `CordError` struct encapsulates an error, providing detailed information
/// about the error kind and its source.
#[derive(Debug)]
pub struct CordError {
    kind: CordErrorKind,
    source: Box<dyn Error + Send + Sync + 'static>,
}

impl CordError {
    /// Creates a new `CordError` instance.
    pub fn new<E>(kind: CordErrorKind, error: E) -> Self
    where
        E: Into<Box<dyn Error + Send + Sync + 'static>>,
    {
        Self {
            kind,
            source: error.into(),
        }
    }
}

impl fmt::Display for CordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Kind: {:?}, Source: {:?}", self.kind, self.source)
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

/// Alias for `std::result::Result<T, CordError>`.
pub type Result<T> = std::result::Result<T, CordError>;
