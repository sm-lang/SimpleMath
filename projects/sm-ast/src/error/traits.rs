use crate::SMError;
use std::io::Error;
use sm_parser::Rule;

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


