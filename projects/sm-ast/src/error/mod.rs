mod traits;

#[derive(Debug, Clone)]
pub enum SMError {
    IOError(String),
    ParseError(String),
    EmptyContainer(String),
    Overflow,
    Infinity,
    ComplexInfinity,
    Unimplemented(String),
    Unreachable(String),
}

pub type SMResult<T> = Result<T, SMError>;
