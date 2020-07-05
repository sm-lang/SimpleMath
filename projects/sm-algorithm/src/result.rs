use num::{BigInt, BigUint};
use std::{
    fmt,
    fmt::{Display, Formatter},
};

/// something like result but flat
#[derive(Debug)]
pub enum Output {
    Integer(BigInt),
    OverFlow,
    ComplexInfinity,
    IOError(String),
}

impl Display for Output {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Output::Integer(i) => write!(f, "{}", i),
            Output::OverFlow => write!(f, "OverFlow"),
            Output::ComplexInfinity => write!(f, "ComplexInfinity"),
            Output::IOError(s) => write!(f, "IOError: {}", s),
        }
    }
}

impl From<BigUint> for Output {
    fn from(u: BigUint) -> Self {
        Output::Integer(BigInt::from(u))
    }
}
