use crate::SMError;
use sm_algorithm::Error as Algorithm;
use sm_parser::Rule;
use std::io::Error;

impl From<std::io::Error> for SMError {
    fn from(e: Error) -> Self {
        SMError::IOError(e.to_string())
    }
}

impl From<sm_parser::Error<Rule>> for SMError {
    fn from(e: sm_parser::Error<Rule>) -> Self {
        SMError::ParseError(e.to_string())
    }
}

impl From<sm_algorithm::Error> for SMError {
    fn from(e: Algorithm) -> Self {
        match e {
            Algorithm::OverFlow => SMError::Overflow,
            Algorithm::ComplexInfinity => SMError::ComplexInfinity,
            Algorithm::IOError(s) => SMError::IOError(s),
            _ => SMError::Unimplemented,
        }
    }
}
