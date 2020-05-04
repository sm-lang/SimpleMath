use num::BigInt;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub enum AST {
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
    Integer(BigInt),
    Decimal(f64),
    Symbol(Symbol),
    String(Box<str>),
}

#[derive(Clone, Debug)]
pub struct Symbol {
    pub name: Box<str>,
}
