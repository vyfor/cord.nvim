#![allow(unused)]

pub mod deserialize;
pub mod serialize;
pub mod value;

pub use deserialize::Deserialize;
pub use serialize::Serialize;
pub use serialize::SerializeFn;
pub use serialize::SerializeObj;
pub use serialize::SerializeState;
pub use value::Value;
pub use value::ValueRef;

#[derive(Debug)]
pub enum Error {
    InvalidUtf8,
    InvalidSyntax(&'static str),
    UnexpectedChar(char),
    UnexpectedEnd,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidUtf8 => write!(f, "invalid utf8"),
            Error::InvalidSyntax(msg) => write!(f, "invalid syntax: {}", msg),
            Error::UnexpectedChar(c) => write!(f, "unexpected character: '{}'", c),
            Error::UnexpectedEnd => write!(f, "unexpected end of input"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::string::FromUtf8Error> for Error {
    fn from(_: std::string::FromUtf8Error) -> Self {
        Error::InvalidUtf8
    }
}

pub struct Json;
