// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
use crate::Error;
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};
use std::collections::HashMap;
use std::fmt;

pub enum Value {
    NULL,

    String(String),

    Array(Vec<Value>),

    Object(HashMap<String, Value>),
}

impl Value {
    pub fn push(&mut self, value: Value) -> Result<(), Error> {
        match self {
            Value::Array(vec) => vec.push(value),
            _ => return Err(Error::ErrNotSupported),
        };

        Ok(())
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::NULL
    }
}

/// The customized JSON converter
impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::NULL => serializer.serialize_none(),
            Value::String(s) => serializer.serialize_str(&s),
            Value::Array(vec) => {
                let mut seq = serializer.serialize_seq(Some(vec.len()))?;

                for elem in vec {
                    seq.serialize_element(elem)?;
                }

                seq.end()
            }
            Value::Object(map) => {
                let mut seq = serializer.serialize_map(Some(map.len()))?;

                for (k, v) in map.iter() {
                    seq.serialize_entry(k, v)?;
                }

                seq.end()
            }
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Value {
        Value::String(value.to_string())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Value {
        Value::String(value.to_string())
    }
}

// vim: set ts=4 sw=4 expandtab:
