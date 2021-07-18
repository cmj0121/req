// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
//! The kind of the error message.
use std::fmt;

/// The pre-define and/or customized error message.
#[derive(Debug, Clone)]
pub enum Error {
    /// Not implemented.
    NotImplemented,
    /// Invalid regex pattern.
    InvalidRegex(String),
    /// The custimized error message.
    Message(String),
}

/// Format trait for an empty format for Error.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidRegex(s) => write!(f, "InvalidRegex: {}", s),
            Error::Message(s) => write!(f, "Error: {}", s),
            _ => write!(f, "{:?}", self),
        }
    }
}

// vim: set ts=4 sw=4 expandtab:
