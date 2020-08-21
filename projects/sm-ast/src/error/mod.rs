mod traits;

#[derive(Debug, Clone)]
pub enum SMError {
    Unknown,
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

#[macro_export]
macro_rules! unimplemented_function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        format!("Unimplemented Function: {} at line {}", &name[..name.len() - 3], line!())
    }};
}
