use crate::AST;

mod check_attributes;
mod traits;

pub trait CheckAttributes {
    fn is_function(&self) -> bool {
        false
    }
    fn is_string(&self) -> bool {
        false
    }
    fn is_power(&self) -> bool {
        false
    }
    fn is_number(&self) -> bool {
        false
    }
    fn is_complex(&self) -> bool {
        false
    }
    fn is_integer(&self) -> bool {
        false
    }
    fn is_positive(&self) -> bool {
        false
    }
    fn is_negative(&self) -> bool {
        false
    }
    fn is_zero(&self) -> bool {
        false
    }
    fn is_one(&self) -> bool {
        false
    }
    fn is_negative_one(&self) -> bool {
        false
    }
    fn is_boolean(&self) -> bool {
        false
    }
    fn is_null(&self) -> bool {
        false
    }
}

pub fn check_symbol_alias(node: &AST, rhs:&str) ->bool{
    match node {
        AST::Symbol(s) => {
            s.name == rhs
        },
        _ => false,
    }
}

pub fn check_function_name(node: &AST, rhs:&str) ->bool{
    match node {
        AST::FunctionCall { name,.. } => {
            match *name.clone() {
                AST::Symbol(s) => {
                    s.name == rhs
                },
                _ =>false
            }
        },
        _ => false,
    }
}