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
pub struct MsgPack;

pub const NIL: u8 = 0xc0;
pub const FALSE: u8 = 0xc2;
pub const TRUE: u8 = 0xc3;
pub const INT8: u8 = 0xd0;
pub const INT16: u8 = 0xd1;
pub const INT32: u8 = 0xd2;
pub const INT64: u8 = 0xd3;
pub const FLOAT32: u8 = 0xca;
pub const FLOAT64: u8 = 0xcb;
pub const STR16: u8 = 0xda;
pub const STR32: u8 = 0xdb;
pub const ARRAY16: u8 = 0xdc;
pub const ARRAY32: u8 = 0xdd;
pub const MAP16: u8 = 0xde;
pub const MAP32: u8 = 0xdf;
pub const UINT8: u8 = 0xcc;
pub const UINT16: u8 = 0xcd;
pub const UINT32: u8 = 0xce;
pub const UINT64: u8 = 0xcf;

pub const POSITIVE_FIXINT_MASK: u8 = 0x80;
pub const POSITIVE_FIXINT_VALUE: u8 = 0x00;

pub const NEGATIVE_FIXINT_MASK: u8 = 0xe0;
pub const NEGATIVE_FIXINT_VALUE: u8 = 0xe0;

pub const FIXSTR_MASK: u8 = 0xe0;
pub const FIXSTR_VALUE: u8 = 0xa0;
pub const FIXSTR_SIZE_MASK: u8 = 0x1f;

pub const FIXARRAY_MASK: u8 = 0xf0;
pub const FIXARRAY_VALUE: u8 = 0x90;
pub const FIXARRAY_SIZE_MASK: u8 = 0x0f;

pub const FIXMAP_MASK: u8 = 0xf0;
pub const FIXMAP_VALUE: u8 = 0x80;
pub const FIXMAP_SIZE_MASK: u8 = 0x0f;

#[derive(Debug)]
pub enum Error {
    InvalidUtf8,
    InvalidLength,
    InvalidMapKey,
    UnexpectedByte(u8),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidUtf8 => write!(f, "invalid utf8"),
            Error::InvalidLength => write!(f, "invalid length"),
            Error::InvalidMapKey => write!(f, "map key must be string"),
            Error::UnexpectedByte(b) => write!(f, "unexpected byte: {:#x}", b),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::string::FromUtf8Error> for Error {
    fn from(_: std::string::FromUtf8Error) -> Self {
        Error::InvalidUtf8
    }
}
