use std::fmt;

#[derive(Debug)]
pub enum Error {
    UnknownUnit(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::UnknownUnit(input) => write!(f, "Unknown unit {input}"),
        }
    }
}

impl std::error::Error for Error {}
