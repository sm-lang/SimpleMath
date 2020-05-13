use bigdecimal::BigDecimal;
use num::BigInt;
use std::collections::BTreeMap;

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
    FunctionCall {
        name: Box<AST>,
        arguments: Vec<AST>,
        options: BTreeMap<AST, AST>,
        position: Position,
    },
    MultiplicativeExpression {
        expressions:Vec<AST>,
        position: Position,
    },
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
    Decimal(BigDecimal),
    Symbol(Symbol),
    String(String),
}

impl AST {
    pub fn integer(i: impl Into<BigInt>) -> AST {
        AST::Integer(i.into())
    }

    pub fn symbol(s: &str) -> AST {
        let mut ns: Vec<_> = s.split("::").map(String::from).collect();
        let n = ns.pop().unwrap();
        AST::Symbol(Symbol { name_space: ns, name: n })
    }

    pub fn string(s: impl Into<String>) -> AST {
        AST::String(s.into())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub file: String,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Symbol {
    pub name_space: Vec<String>,
    pub name: String,
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
