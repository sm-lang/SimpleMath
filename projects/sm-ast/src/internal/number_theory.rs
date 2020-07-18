use crate::{SMResult, AST};
use sm_algorithm::prime_sum_i;

pub fn floor(expr: &AST) -> SMResult<AST> {
    match expr {
        _ => unimplemented!(),
    }
}

pub fn ceiling(expr: &AST) -> SMResult<AST> {
    match expr {
        _ => unimplemented!(),
    }
}

pub fn round(expr: &AST) -> SMResult<AST> {
    match expr {
        _ => unimplemented!(),
    }
}

pub fn integer_part(expr: &AST) -> SMResult<AST> {
    match expr {
        _ => unimplemented!(),
    }
}

pub fn chop(expr: &AST) -> SMResult<AST> {
    match expr {
        _ => unimplemented!(),
    }
}

pub fn factor_integer(expr: &AST) -> SMResult<AST> {
    match expr {
        _ => unimplemented!(),
    }
}

pub fn prime_sum(expr: &AST) -> SMResult<AST> {
    match expr {
        AST::Integer(n) => Ok(AST::Integer(prime_sum_i(n)?)),
        AST::Decimal(n) => Ok(AST::Decimal(n.clone())),
        _ => unimplemented!(),
    }
}
