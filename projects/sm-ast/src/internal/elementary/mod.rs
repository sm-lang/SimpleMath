use crate::{SMError, SMResult, AST};
use sm_algorithm::{factorial_i, Output, fibonacci_i};

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
        AST::Integer(n) => match factorial_i(n) {
            Output::Integer(n) => Ok(AST::integer(n)),
            Output::OverFlow => Ok(AST::symbol("std::constant::OverFlow")),
            Output::ComplexInfinity => Ok(AST::symbol("std::constant::ComplexInfinity")),
            Output::IOError(e) => Err(SMError::IOError(e)),
        },
        AST::Decimal(n) => Ok(AST::Decimal(n.clone())),
        _ => unimplemented!(),
    }
}

pub fn fibonacci(expr: &AST) -> SMResult<AST> {
    match expr {
        AST::Integer(n) => match fibonacci_i(n){
            Output::Integer(n) => Ok(AST::integer(n)),
            Output::OverFlow => Ok(AST::symbol("std::constant::OverFlow")),
            Output::ComplexInfinity => Ok(AST::symbol("std::constant::ComplexInfinity")),
            Output::IOError(e) => Err(SMError::IOError(e)),
        },
        AST::Decimal(n) => Ok(AST::Decimal(n.clone())),
        _ => unimplemented!(),
    }
}

pub fn power_mod(expr: &AST) -> SMResult<AST> {
    match expr {
        _ => unimplemented!(),
    }
}
