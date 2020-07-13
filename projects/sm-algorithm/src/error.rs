use std::{
    fmt,
    fmt::{Display, Formatter},
    result,
};

#[derive(Debug)]
pub enum Error {
    OverFlow,
    ComplexInfinity,
    IOError(String),
    Unimplemented,
    Undefined(String),
}

pub type Result<T> = result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::OverFlow => write!(f, "OverFlow"),
            Error::ComplexInfinity => write!(f, "ComplexInfinity"),
            Error::IOError(s) => write!(f, "IOError: {}", s),
            Error::Unimplemented => write!(f, "Unimplemented"),
            Error::Undefined(s) => write!(f, "Undefined: {}", s),
        }
    }
}
