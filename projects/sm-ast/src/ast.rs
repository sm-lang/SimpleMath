use num::BigInt;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub enum AST {
    EmptyStatement,
    NewLine,
    Expression {
        base: Box<AST>,
        eos: bool,
        pos: Position,
    },
    //
    /// function call
    /// function(name, *args, **kwargs)
    Function(String, Vec<AST>, BTreeMap<AST, AST>),

    //
    /// unary operators
    UnaryOperators {
        base: Box<AST>,
        prefix: Vec<String>,
        suffix: Vec<String>,
        position: Position,
    },
    /// - `InfixOperators`
    InfixOperators {
        infix: String,
        lhs: Box<AST>,
        rhs: Box<AST>,
        position: Position,
    },

    //
    /// the source of all evil
    Null,
    /// true or false
    Boolean(bool),
    Integer(BigInt),
    Decimal(f64),
    Symbol(Symbol),
    String(String),
}

impl AST {
    pub fn integer(i: impl Into<BigInt>) -> AST {
        AST::Integer(i.into())
    }

    pub fn symbol(s: &str) -> AST {
        AST::Symbol(Symbol { name: Box::from(s), ..Default::default() })
    }

    pub fn string(s: impl Into<String>) -> AST {
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
