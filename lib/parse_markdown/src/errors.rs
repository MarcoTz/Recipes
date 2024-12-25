use super::parse_steps::ParseStep;
use recipes::errors::Error as RecipeError;
use std::{fmt, io::ErrorKind};

#[derive(Debug)]
pub enum Error {
    IO(ErrorKind),
    MissingHeader(String),
    MissingPart(String, ParseStep),
    Parse(String, ParseStep),
    RecipeError(RecipeError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IO(kind) => write!(f, "Encountered an IO error: {kind}"),
            Error::MissingHeader(name) => write!(f, "Input is missing header {name}"),
            Error::MissingPart(name, step) => write!(f, "Missing {name} in {step}"),
            Error::Parse(name, step) => write!(f, "Could not parse {name} in {step}"),
            Error::RecipeError(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(io_err: std::io::Error) -> Error {
        Error::IO(io_err.kind())
    }
}

impl From<RecipeError> for Error {
    fn from(err: RecipeError) -> Error {
        Error::RecipeError(err)
    }
}
