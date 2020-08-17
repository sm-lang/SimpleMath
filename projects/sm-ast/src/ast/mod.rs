use bigdecimal::BigDecimal;
use num::BigInt;
use std::collections::BTreeMap;
mod traits;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AST {
    EmptyStatement,
    NewLine,
    //
    Program(Vec<AST>),
    Expression {
        base: Box<AST>,
        eos: bool,
        position: Position,
    },
    //
    /// function call
    /// function(name, *args, **kwargs)
    Function(Symbol, Vec<Parameter>),
    //
    /// the source of all evil
    Null,
    /// true or false
    Boolean(bool),
    Integer(BigInt),
    Decimal(BigDecimal),
    Symbol(Symbol),
    String(String),
}

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
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Parameter {
    pub arguments: Vec<AST>,
    pub options: BTreeMap<AST, AST>,
    pub position: Position,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub file: String,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Symbol {
    pub name_space: Vec<String>,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Expression {}
