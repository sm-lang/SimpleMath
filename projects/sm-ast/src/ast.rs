use num::BigInt;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub enum AST {
    EmptyStatement,
    NewLine,

    //
    /// function call
    /// function(name, *args, **kwargs)
    Function(Box<str>, Vec<AST>, BTreeMap<AST, AST>),

    //
    /// prefix operation
    Prefix(Box<str>, Box<AST>),
    /// suffix operation
    Suffix(Box<str>, Box<AST>),
    /// binary operation
    Binary(Box<str>, Box<AST>, Box<AST>),

    //
    /// the source of all evil
    Null,
    /// true or false
    Boolean(bool),
    Integer {
        value: BigInt,
        position: Option<Position>,
    },
    Decimal(f64),
    Symbol(Symbol),
    String(Box<str>),
}

impl AST {
    pub fn integer(i: impl Into<BigInt>) -> AST {
        AST::Integer {
            value: i.into(),
            position: None,
        }
    }

    pub fn symbol(s: &str) -> AST {
        AST::Symbol(Symbol { name: Box::from(s), ..Default::default() })
    }

    pub fn string(s: impl Into<Box<str>>) -> AST {
        AST::String(s.into())
    }
}

#[derive(Clone, Debug)]
pub struct Position {
    pub file: String,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

#[derive(Clone, Debug, Default)]
pub struct Symbol {
    pub name: Box<str>,
}

#[derive(Clone, Debug, Default)]
pub struct Expression {}

pub trait CheckAttributes {
    fn is_function(&self) -> bool {
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
}
