// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
use crate::{Error, Value};
use log::info;
use regex::{Captures, Regex};
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

    pub fn parse(&self, text: &str, single: bool) -> Result<Value, Error> {
        match single {
            true => match self.re.captures(text) {
                Some(caps) => self.parse_captures(caps),
                None => Ok(Value::default()),
            },
            false => {
                let mut vec = Value::Array(vec![]);

                for caps in self.re.captures_iter(text) {
                    let value = self.parse_captures(caps)?;
                    vec.push(value)?;
                }

                Ok(vec)
            }
        }
    }

    fn parse_captures(&self, caps: Captures) -> Result<Value, Error> {
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

        Ok(value)
    }
}

// vim: set ts=4 sw=4 expandtab:
