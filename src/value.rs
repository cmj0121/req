// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
use crate::error::Error;
use crate::query::{Query, QueryMode};
use log::trace;
use regex::{Captures, Regex};
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
    pub fn new(f: Option<PathBuf>, regex: &str, query: &str) -> Result<Self, Error> {
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

        Value::from_str(&text, regex, query)
    }

    /// create a new query instance from &str.
    pub fn from_str(text: &str, re: &str, query: &str) -> Result<Self, Error> {
        trace!("from_str text: {}", text);

        match Regex::new(re) {
            Err(err) => Err(Error::InvalidInput(format!(
                "regex: {} invalid:{}",
                re, err
            ))),
            Ok(re) => {
                let query = Query::new(query)?;

                match query.single_mode {
                    true => match re.captures(text) {
                        Some(caps) => Ok(Value::from_capture(caps, &query, &re)),
                        None => Ok(Value::NULL),
                    },
                    false => {
                        let mut obj: Vec<Value> = vec![];

                        for caps in re.captures_iter(text) {
                            // save the value into Value::Array
                            obj.push(Value::from_capture(caps, &query, &re));
                        }

                        Ok(Value::Array(obj))
                    }
                }
            }
        }
    }

    /// parse the value from the regex::Captures
    fn from_capture(caps: Captures, query: &Query, re: &Regex) -> Self {
        match caps.get(0) {
            Some(matched) => {
                match query.query_mode {
                    QueryMode::Full => Value::String(matched.as_str().to_string()),
                    QueryMode::Group => {
                        let mut obj: Vec<Value> = vec![];

                        for sub in caps.iter().skip(1) {
                            // save the value into Value::Array
                            match sub {
                                Some(sub_matched) => {
                                    obj.push(Value::String(sub_matched.as_str().to_string()))
                                }
                                None => obj.push(Value::NULL),
                            };
                        }

                        Value::Array(obj)
                    }
                    QueryMode::Named => {
                        let mut obj: HashMap<String, Value> = HashMap::new();

                        for name in re.capture_names().flatten() {
                            // save the value into Value::Object
                            match caps.name(name) {
                                Some(sub_matched) => obj.insert(
                                    name.to_string(),
                                    Value::String(sub_matched.as_str().to_string()),
                                ),
                                None => obj.insert(name.to_string(), Value::NULL),
                            };
                        }

                        Value::Object(obj)
                    }
                }
            }
            None => Value::NULL,
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
