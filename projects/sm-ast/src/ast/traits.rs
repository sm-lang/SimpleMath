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
use crate::ast::SymbolKind;

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
    pub fn prefix(s: &str, o: &str) -> AST {
        let mut s = Symbol::from(s);
        s.kind == SymbolKind::Prefix(Box::from(o));
        AST::Symbol(s)
    }

    pub fn infix(s: &str, o: &str, p: u8) -> AST {
        let mut s = Symbol::from(s);
        s.kind == SymbolKind::Infix(Box::from(o), p);
        AST::Symbol(s)
    }
    pub fn suffix(s: &str, o: &str) -> AST {
        let mut s = Symbol::from(s);
        s.kind == SymbolKind::Suffix(Box::from(o));
        AST::Symbol(s)
    }

}

impl From<&str> for Symbol {
    fn from(s: &str) -> Self {
        let mut ns = s.split("::").map(String::from).collect_vec();
        let n = ns.pop().unwrap();
        Symbol { name_space: ns, name: n, kind: SymbolKind::Normal, attributes: 0 }
    }
}

impl From<String> for Symbol {
    fn from(s: String) -> Self {
        Symbol::from(s.as_ref())
    }
}

impl Default for Symbol {
    fn default() -> Self {
        Self { name_space: vec![], name: "".to_string(), kind: SymbolKind::Normal, attributes: 0 }
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
            AST::EmptyStatement => { unimplemented!() }
            AST::Program(_) => { unimplemented!() }
            AST::Function(_, _) => { unimplemented!() }
            #[rustfmt::skip]
            AST::Boolean(b) => if *b { write!(f, "true") } else { write!(f, "false") }
            AST::Integer(n) => write!(f, "{}", n),
            AST::Decimal(n) => write!(f, "{}", n),
            AST::Symbol(s) => write!(f, "{}", s),
            AST::String(s) => write!(f, "{}", s),
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
            AST::Symbol(s) => s.name == "Null",
            _ => false,
        }
    }
}
