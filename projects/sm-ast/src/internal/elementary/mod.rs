use crate::{SMResult, AST};
use sm_algorithm::{factorial_i, fibonacci_i};

pub fn sqrt(expr: &AST) -> SMResult<AST> {
    match expr {
        _ => unimplemented!(),
    }
}

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

pub fn log(expr: &AST) -> SMResult<AST> {
    match expr {
        _ => unimplemented!(),
    }
}

pub fn log2(expr: &AST) -> SMResult<AST> {
    match expr {
        _ => unimplemented!(),
    }
}
pub fn log10(expr: &AST) -> SMResult<AST> {
    match expr {
        _ => unimplemented!(),
    }
}

pub fn factorial(expr: &AST) -> SMResult<AST> {
    match expr {
        AST::Integer(n) => Ok(AST::Integer(factorial_i(n)?)),
        AST::Decimal(n) => Ok(AST::Decimal(n.clone())),
        _ => unimplemented!(),
    }
}

pub fn fibonacci(expr: &AST) -> SMResult<AST> {
    match expr {
        AST::Integer(n) => Ok(AST::Integer(fibonacci_i(n)?)),
        AST::Decimal(n) => Ok(AST::Decimal(n.clone())),
        _ => unimplemented!(),
    }
}

pub fn power_mod(expr: &AST) -> SMResult<AST> {
    match expr {
        _ => unimplemented!(),
    }
}
