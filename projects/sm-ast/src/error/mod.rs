pub enum SMError {
    EmptyContainer(String)
}

pub type SMResult<T> = Result<T, SMError>;
