use std::{fmt, io::ErrorKind};

#[derive(Debug)]
pub enum Error {
    IO(ErrorKind),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IO(kind) => write!(f, "Encountered an IO error: {kind}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(io_err: std::io::Error) -> Error {
        Error::IO(io_err.kind())
    }
}
