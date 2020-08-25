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
    Indeterminate,
    Undefined(String),
}

pub type Result<T> = result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::IOError(s) => write!(f, "IOError: {}", s),
            Error::Undefined(s) => write!(f, "Undefined: {}", s),
            _ => write!(f,"{:?}",self)
        }
    }
}
