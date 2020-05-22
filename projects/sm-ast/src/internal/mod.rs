use crate::{AST, SMResult};
use crate::error::SMError::EmptyContainer;

pub fn head(expr: &AST)->AST {
    match expr {
        AST::Expression { .. } => {unimplemented!()}
        AST::FunctionCall { .. } => {unimplemented!()}
        AST::MultiplicativeExpression { .. } => AST::symbol("Multiplicative"),
        AST::AdditiveExpression { .. } => AST::symbol("Additive"),
        AST::List(..) => AST::symbol("List"),
        AST::UnaryOperators { .. } => {unimplemented!()}
        AST::InfixOperators { .. } => {unimplemented!()}
        AST::Boolean(..) => AST::symbol("Boolean"),
        AST::Integer(..) => AST::symbol("Integer"),
        AST::Decimal(..) => AST::symbol("Decimal"),
        AST::Symbol(..) => AST::symbol("Symbol"),
        AST::String(..) => AST::symbol("String"),
        _ => AST::Null
    }
}

pub fn first(expr: &AST) -> SMResult<AST> {
    match expr {
        AST::List(v) => {
            match v.get(0) {
                None => Err(EmptyContainer(String::from("Can't call first on empty list"))),
                Some(s) => { Ok(s.clone()) }
            }
        }
        AST::String(s) => {
            match s.chars().next() {
                None => Err(EmptyContainer(String::from("Can't call first on empty string"))),
                Some(s) => {Ok(AST::string(s))}
            }
        }
        _ => unimplemented!()
    }
}

pub fn last(expr: &AST) -> SMResult<AST> {
    match expr {
        AST::List(v) => {
            match v.iter().rev().next() {
                None => Err(EmptyContainer(String::from("Can't call `last` on empty list"))),
                Some(s) => { Ok(s.clone()) }
            }
        }
        AST::String(s) => {
            match s.chars().rev().next() {
                None => Err(EmptyContainer(String::from("Can't call `last` on empty string"))),
                Some(s) => {Ok(AST::string(s))}
            }
        }
        _ => unimplemented!()
    }
}