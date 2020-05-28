mod traits;

#[derive(Debug, Clone)]
pub enum SMError {
    EmptyContainer(String),
    Overflow,
}

pub type SMResult<T> = Result<T, SMError>;
