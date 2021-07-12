// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
use crate::{Error, Value};
use log::info;
use regex::Regex;
use std::collections::HashMap;

pub struct RegexQuery {
    re: Regex,
}

impl RegexQuery {
    pub fn new(re: &str) -> Result<Self, Error> {
        match Regex::new(re) {
            Ok(re) => Ok(Self { re: re }),
            Err(err) => {
                info!("cannot generate re from {:?}: {:?}", re, err);
                Err(Error::ErrRegexPattern)
            }
        }
    }

    pub fn parse(&self, text: &str) -> Result<Value, Error> {
        let mut vec: Vec<Value> = vec![];

        for caps in self.re.captures_iter(text) {
            // global matched
            let mut value = Value::Array(vec![Value::from(&caps[0])]);

            // sub-groups
            let mut sub_value = Value::Array(vec![]);
            for sub in caps.iter().skip(1) {
                match sub {
                    Some(matched) => sub_value.push(Value::from(matched.as_str()))?,
                    None => sub_value.push(Value::default())?,
                }
            }
            value.push(sub_value)?;

            // named-groups
            let mut named_value: HashMap<String, Value> = HashMap::new();
            for name in self.re.capture_names().flatten() {
                match caps.name(name) {
                    Some(matched) => {
                        named_value.insert(name.to_string(), Value::from(matched.as_str()))
                    }
                    None => named_value.insert(name.to_string(), Value::default()),
                };
            }
            value.push(Value::Object(named_value))?;

            vec.push(value);
        }

        Ok(Value::Array(vec))
    }
}

// vim: set ts=4 sw=4 expandtab:
