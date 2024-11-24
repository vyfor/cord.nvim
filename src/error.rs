use std::error::Error;
use std::fmt;
use std::io;
use std::num::ParseIntError;
use std::string::ParseError;

use crate::json;
use crate::msgpack;

#[derive(Debug)]
pub enum CordError {
    Io(io::Error),
    Parse(CordParseError),
    MsgPack(msgpack::Error),
    Json(json::Error),
    Other(String),
}

#[derive(Debug)]
pub enum CordParseError {
    ParseError(ParseError),
    ParseIntError(ParseIntError),
}

impl fmt::Display for CordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CordError::Io(err) => write!(f, "IO error: {}", err),
            CordError::Parse(err) => write!(f, "Parse error: {}", err),
            CordError::MsgPack(err) => write!(f, "MsgPack error: {}", err),
            CordError::Json(err) => write!(f, "JSON error: {}", err),
            CordError::Other(err) => write!(f, "Error: {}", err),
        }
    }
}

impl fmt::Display for CordParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CordParseError::ParseError(err) => write!(f, "Parse error: {}", err),
            CordParseError::ParseIntError(err) => write!(f, "Parse int error: {}", err),
        }
    }
}

impl Error for CordError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CordError::Io(err) => Some(err),
            CordError::Parse(err) => Some(err),
            CordError::MsgPack(err) => Some(err),
            CordError::Json(err) => Some(err),
            _ => None,
        }
    }
}

impl Error for CordParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CordParseError::ParseError(err) => Some(err),
            CordParseError::ParseIntError(err) => Some(err),
        }
    }
}

impl From<msgpack::Error> for CordError {
    fn from(err: msgpack::Error) -> Self {
        CordError::MsgPack(err)
    }
}

impl From<json::Error> for CordError {
    fn from(err: json::Error) -> Self {
        CordError::Json(err)
    }
}

impl From<io::Error> for CordError {
    fn from(err: io::Error) -> Self {
        CordError::Io(err)
    }
}

impl From<ParseError> for CordError {
    fn from(err: ParseError) -> Self {
        CordError::Parse(CordParseError::ParseError(err))
    }
}

impl From<ParseIntError> for CordError {
    fn from(err: ParseIntError) -> Self {
        CordError::Parse(CordParseError::ParseIntError(err))
    }
}

impl From<&str> for CordError {
    fn from(err: &str) -> Self {
        CordError::Other(err.to_string())
    }
}

impl From<String> for CordError {
    fn from(err: String) -> Self {
        CordError::Other(err)
    }
}

pub type Result<T> = std::result::Result<T, CordError>;
