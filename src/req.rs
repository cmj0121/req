// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
use crate::Error;
use log::trace;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

/// The instance that generate the query for regular-expression.
pub struct Query {}

impl Query {
    /// create a new query instance from passed file path.
    pub fn new(f: Option<PathBuf>) -> Result<Self, Error> {
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

        Query::from_str(&text)
    }

    /// create a new query instance from &str.
    pub fn from_str(text: &str) -> Result<Self, Error> {
        trace!("from_str: {}", text);
        Err(Error::NotImplemented)
    }
}

// vim: set ts=4 sw=4 expandtab:
