use crate::{
    error::SMError::{ComplexInfinity, IOError, Overflow, ParseError},
    SMError,
};
use sm_algorithm::Error as Algorithm;
use sm_parser::{Error as Parser, Rule};
use std::io::Error;

impl From<std::io::Error> for SMError {
    fn from(e: Error) -> Self {
        IOError(e.to_string())
    }
}

impl From<Parser<Rule>> for SMError {
    fn from(e: Parser<Rule>) -> Self {
        ParseError(e.to_string())
    }
}

impl From<Algorithm> for SMError {
    fn from(e: Algorithm) -> Self {
        match e {
            Algorithm::OverFlow => Overflow,
            Algorithm::ComplexInfinity => ComplexInfinity,
            Algorithm::IOError(s) => IOError(s),
            _ => SMError::Unimplemented,
        }
    }
}
