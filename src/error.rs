// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
//! The kind of the error message.

/// The pre-define and/or customized error message.
#[derive(Debug, Clone)]
pub enum Error {
    /// Not implemented.
    NotImplemented,
    /// The custimized error message.
    Message(String),
}

// vim: set ts=4 sw=4 expandtab:
