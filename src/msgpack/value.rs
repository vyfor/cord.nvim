use std::collections::HashMap;

use super::{serialize::Serialize, NIL};

#[derive(Debug, PartialEq)]
pub enum Value {
    Nil,
    Boolean(bool),
    Integer(i64),
    UInteger(u64),
    Float(f64),
    String(String),
    Binary(Vec<u8>),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
}

#[repr(u8)]
pub enum ValueRef<'a> {
    Nil = NIL,
    String(&'a str),
    Float(f64),
    Integer(i64),
    UInteger(u64),
    Boolean(bool),
    Array(Vec<ValueRef<'a>>),
    Object(&'a dyn Serialize),
}

impl Value {
    #[inline]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    #[inline]
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            Value::Binary(b) => Some(b),
            _ => None,
        }
    }

    #[inline]
    pub fn as_array(&self) -> Option<&[Value]> {
        match self {
            Value::Array(arr) => Some(arr),
            _ => None,
        }
    }

    #[inline]
    pub fn as_map(&self) -> Option<&HashMap<String, Value>> {
        match self {
            Value::Map(map) => Some(map),
            _ => None,
        }
    }

    #[inline]
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Value::Integer(n) => Some(*n),
            Value::UInteger(n) if *n <= i64::MAX as u64 => Some(*n as i64),
            _ => None,
        }
    }

    #[inline]
    pub fn as_uinteger(&self) -> Option<u64> {
        match self {
            Value::UInteger(n) => Some(*n),
            Value::Integer(n) if *n >= 0 => Some(*n as u64),
            _ => None,
        }
    }

    #[inline]
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(n) => Some(*n),
            _ => None,
        }
    }

    #[inline]
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    #[inline]
    pub fn is_nil(&self) -> bool {
        matches!(self, Value::Nil)
    }

    #[inline]
    pub fn take_string(self) -> Option<String> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    #[inline]
    pub fn take_bytes(self) -> Option<Vec<u8>> {
        match self {
            Value::Binary(b) => Some(b),
            _ => None,
        }
    }

    #[inline]
    pub fn take_array(self) -> Option<Vec<Value>> {
        match self {
            Value::Array(arr) => Some(arr),
            _ => None,
        }
    }

    #[inline]
    pub fn take_map(self) -> Option<HashMap<String, Value>> {
        match self {
            Value::Map(map) => Some(map),
            _ => None,
        }
    }
}
