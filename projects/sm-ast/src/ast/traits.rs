use crate::{
    ast::{CheckAttributes, Position, Symbol},
    AST,
};
use bigdecimal::BigDecimal;
use itertools::Itertools;
use num::{BigInt, One, Zero};
use std::{
    fmt,
    fmt::{Display, Formatter},
};

impl AST {
    pub fn integer(n: impl Into<BigInt>) -> AST {
        AST::Integer(n.into())
    }
    pub fn decimal(n: impl Into<BigDecimal>) -> AST {
        AST::Decimal(n.into())
    }

    pub fn symbol(s: impl AsRef<str>) -> AST {
        AST::Symbol(Symbol::from(s.as_ref()))
    }

    pub fn string(s: impl Into<String>) -> AST {
        AST::String(s.into())
    }
}

impl From<&str> for Symbol {
    fn from(s: &str) -> Self {
        let mut ns: Vec<_> = s.split("::").map(String::from).collect();
        let n = ns.pop().unwrap();
        Symbol { name_space: ns, name: n }
    }
}

impl Default for Symbol {
    fn default() -> Self {
        Self { name_space: vec![], name: "".to_string() }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self { file: "".to_string(), start: (0, 0), end: (0, 0) }
    }
}

#[allow(unused_must_use)]
impl Display for AST {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            AST::Null => write!(f, "null"),
            AST::Expression { base, eos, .. } => {
                write!(f, "{}", base);
                if *eos { write!(f, ";") } else { write!(f, "") }
            }
            // AST::List(v) => {
            // let list = v.iter().map(|e| format!("{}", e)).collect_vec();
            // write!(f, "[{}]", list.join(", "))
            // }
            AST::Boolean(b) => {
                if *b {
                    write!(f, "true")
                }
                else {
                    write!(f, "false")
                }
            }
            AST::Integer(n) => write!(f, "{}", n),
            AST::Decimal(n) => write!(f, "{}", n),
            AST::Symbol(s) => write!(f, "{}", s),
            AST::String(s) => write!(f, "{}", s),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.name_space.len() == 0 { write!(f, "{}", self.name) } else { write!(f, "{}::{}", self.name_space.join("::"), self.name) }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.file)
    }
}

impl CheckAttributes for AST {
    fn is_string(&self) -> bool {
        match &self {
            AST::String(..) => true,
            _ => false,
        }
    }
    fn is_zero(&self) -> bool {
        match self {
            AST::Integer(i) => i.is_zero(),
            AST::Decimal(n) => n.is_zero(),
            _ => false,
        }
    }
    fn is_one(&self) -> bool {
        match self {
            AST::Integer(i) => i.is_one(),
            AST::Decimal(n) => n.is_one(),
            _ => false,
        }
    }
    fn is_boolean(&self) -> bool {
        match &self {
            AST::Boolean(..) => true,
            _ => false,
        }
    }
    fn is_null(&self) -> bool {
        match &self {
            AST::Null => true,
            _ => false,
        }
    }
}
