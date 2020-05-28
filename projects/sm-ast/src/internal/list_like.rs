use crate::{error::SMError::EmptyContainer, SMResult, AST};

pub fn head(expr: &AST) -> AST {
    match expr {
        AST::Expression { .. } => unimplemented!(),
        AST::FunctionCall { .. } => unimplemented!(),
        AST::MultiplicativeExpression { .. } => AST::symbol("Multiplicative"),
        AST::AdditiveExpression { .. } => AST::symbol("Additive"),
        AST::List(..) => AST::symbol("List"),
        AST::UnaryOperators { .. } => unimplemented!(),
        AST::InfixOperators { .. } => unimplemented!(),
        AST::Boolean(..) => AST::symbol("Boolean"),
        AST::Integer(..) => AST::symbol("Integer"),
        AST::Decimal(..) => AST::symbol("Decimal"),
        AST::Symbol(..) => AST::symbol("Symbol"),
        AST::String(..) => AST::symbol("String"),
        _ => AST::Null,
    }
}

pub fn length(expr: &AST) -> SMResult<AST> {
    match expr {
        AST::List(v) => Ok(AST::integer(v.len())),
        AST::String(s) => Ok(AST::integer(s.len())),
        _ => unimplemented!(),
    }
}

pub fn first(expr: &AST) -> SMResult<AST> {
    match expr {
        AST::List(v) => match v.iter().next() {
            None => Err(EmptyContainer(String::from("Can't call `first` on empty list"))),
            Some(s) => Ok(s.clone()),
        },
        AST::String(s) => match s.chars().next() {
            None => Err(EmptyContainer(String::from("Can't call `first` on empty string"))),
            Some(s) => Ok(AST::string(s)),
        },
        _ => unimplemented!(),
    }
}

pub fn last(expr: &AST) -> SMResult<AST> {
    match expr {
        AST::List(v) => match v.iter().rev().next() {
            None => Err(EmptyContainer(String::from("Can't call `last` on empty list"))),
            Some(s) => Ok(s.clone()),
        },
        AST::String(s) => match s.chars().rev().next() {
            None => Err(EmptyContainer(String::from("Can't call `last` on empty string"))),
            Some(s) => Ok(AST::string(s)),
        },
        _ => unimplemented!(),
    }
}

pub fn most(expr: &AST) -> SMResult<AST> {
    match expr {
        _ => unimplemented!(),
    }
}

pub fn rest(expr: &AST) -> SMResult<AST> {
    match expr {
        _ => unimplemented!(),
    }
}

pub fn take(expr: &AST) -> SMResult<AST> {
    match expr {
        _ => unimplemented!(),
    }
}

pub fn join(expr: &AST) -> SMResult<AST> {
    match expr {
        _ => unimplemented!(),
    }
}

pub fn concat(expr: &AST) -> SMResult<AST> {
    match expr {
        _ => unimplemented!(),
    }
}

pub fn flatten(expr: &AST) -> SMResult<AST> {
    match expr {
        _ => unimplemented!(),
    }
}