// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
use crate::Error;
use log::trace;
use regex::Regex;
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

/// The result after the query.
pub enum Value {
    /// The empty of the result, same 'null' in the JSON.
    NULL,
    /// The raw string of the result.
    String(String),
    /// The list of the Value.
    Array(Vec<Value>),
    /// The named of the key-value pair.
    Object(HashMap<String, Value>),
}

impl Value {
    /// create a new query instance from passed file path.
    pub fn new(f: Option<PathBuf>, regex: &str) -> Result<Self, Error> {
        let mut text = String::new();

        match f {
            Some(filename) => match fs::read_to_string(filename.clone()) {
                Ok(raw) => {
                    text = raw;
                }
                Err(err) => {
                    return Err(Error::Message(format!(
                        "cannot read from {:?}: {}",
                        filename, err
                    )))
                }
            },
            None => match io::stdin().read_to_string(&mut text) {
                Ok(_) => {}
                Err(err) => return Err(Error::Message(format!("cannot read from STDIN: {}", err))),
            },
        }

        Value::from_str(&text, regex)
    }

    /// create a new query instance from &str.
    pub fn from_str(text: &str, re: &str) -> Result<Self, Error> {
        trace!("from_str text: {}", text);
        match Regex::new(re) {
            Err(err) => Err(Error::InvalidRegex(format!("{} invalid:{}", re, err))),
            Ok(_) => {
                let obj = Value::NULL;

                Ok(obj)
            }
        }
    }
}

/// The JSON serializer of the Value.
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

// vim: set ts=4 sw=4 expandtab:
