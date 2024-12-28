use std::fmt;

#[derive(Debug)]
pub enum Error {
    UnknownUnit(String),
    BracketMismatch(String),
    MissingLink(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::UnknownUnit(input) => write!(f, "Unknown unit {input}"),
            Error::BracketMismatch(in_str) => write!(f, "Mismatched brackets in {in_str}"),
            Error::MissingLink(text) => write!(f, "Missing link target in {text}"),
        }
    }
}

impl std::error::Error for Error {}
