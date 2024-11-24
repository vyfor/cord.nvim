use std::collections::HashMap;

use super::serialize::Serialize;

#[derive(Debug)]
pub enum Value<'a> {
    String(&'a str),
    Number(f64),
    Bool(bool),
    Null,
    Array(Vec<Value<'a>>),
    Object(HashMap<&'a str, Value<'a>>),
}

#[repr(u8)]
pub enum ValueRef<'a> {
    Null = 0,
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Array(Vec<ValueRef<'a>>),
    Object(&'a dyn Serialize),
}

impl<'a> Value<'a> {
    #[inline]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    #[inline]
    pub fn as_string(&self) -> Option<String> {
        match self {
            Value::String(s) => Some(s.to_string()),
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
    pub fn as_map(&self) -> Option<&HashMap<&str, Value>> {
        match self {
            Value::Object(map) => Some(map),
            _ => None,
        }
    }

    #[inline]
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    #[inline]
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }
}
