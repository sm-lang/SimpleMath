mod utils;
use crate::{SMResult, AST};
pub use utils::*;

pub fn sin(expr: &AST) -> SMResult<AST> {
    match expr {
        _ => unimplemented!(),
    }
}

pub fn cos(expr: &AST) -> SMResult<AST> {
    match expr {
        _ => unimplemented!(),
    }
}

pub fn tan(expr: &AST) -> SMResult<AST> {
    match expr {
        _ => unimplemented!(),
    }
}

pub fn cot(expr: &AST) -> SMResult<AST> {
    match expr {
        _ => unimplemented!(),
    }
}

pub fn sec(expr: &AST) -> SMResult<AST> {
    match expr {
        _ => unimplemented!(),
    }
}

pub fn csc(expr: &AST) -> SMResult<AST> {
    match expr {
        _ => unimplemented!(),
    }
}

pub fn factorial(expr: &AST) -> SMResult<AST> {
    match expr {
        AST::Integer(n) => Ok(AST::integer(factorial_int(n))),
        AST::Decimal(n) => Ok(AST::Decimal(n.clone())),
        _ => unimplemented!(),
    }
}

pub fn fibonacci(expr: &AST) -> SMResult<AST> {
    match expr {
        AST::Integer(n) => Ok(AST::integer(fibonacci_int(n))),
        AST::Decimal(n) => Ok(AST::Decimal(n.clone())),
        _ => unimplemented!(),
    }
}
