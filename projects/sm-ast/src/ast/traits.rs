use crate::{
    ast::{Parameter, Position, Symbol, SymbolKind},
    AST,
};
use bigdecimal::BigDecimal;
use itertools::Itertools;
use num::{BigInt, One, Zero};
use std::{
    fmt,
    fmt::{Display, Formatter},
};

impl Display for AST {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            AST::EmptyStatement => unimplemented!(),
            AST::Program(_) => unimplemented!(),
            AST::Function(s, ps) => {
                match &s.kind {
                    SymbolKind::Normal => unimplemented!(),
                    SymbolKind::Alias => unimplemented!(),
                    SymbolKind::Prefix(o) => {
                        if is_unary(ps) {
                            return write!(f, "{0}{1}", o, ps[0].arguments[0]);
                        }
                    }
                    SymbolKind::Infix(o, p) => {
                        if is_multiary(ps) {
                            let mut v = vec![];
                            for arg in &ps[0].arguments {
                                if arg.precedence() < *p { v.push(format!("({})", arg)) } else { v.push(format!("{}", arg)) }
                            }
                            // let j = if o.as_ref() == "*" { v.join(" ") } else { v.join(&format!(" {} ", o)) };
                            return write!(f, "{0}", v.join(o));
                        }
                    }
                    SymbolKind::Suffix(o) => {
                        if is_unary(ps) {
                            return write!(f, "{1}{0}", o, ps[0].arguments[0]);
                        }
                    }
                }
                write!(f, "")
            }
            #[rustfmt::skip]
            AST::Boolean(b) => if *b { write!(f, "true") } else { write!(f, "false") },
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
    pub(crate) fn precedence(&self) -> u8 {
        if let AST::Function(v, ..) = self {
            if let SymbolKind::Infix(_, p) = v.kind {
                return p;
            }
        }
        return 255;
    }
}

#[allow(dead_code)]
impl AST {
    pub(crate) fn is_string(&self) -> bool {
        match self {
            AST::String(..) => true,
            _ => false,
        }
    }
    pub(crate) fn is_zero(&self) -> bool {
        match self {
            AST::Integer(i) => i.is_zero(),
            AST::Decimal(n) => n.is_zero(),
            _ => false,
        }
    }
    pub(crate) fn is_one(&self) -> bool {
        match self {
            AST::Integer(i) => i.is_one(),
            AST::Decimal(n) => n.is_one(),
            _ => false,
        }
    }
    pub(crate) fn is_boolean(&self) -> bool {
        match self {
            AST::Boolean(..) => true,
            _ => false,
        }
    }
    pub(crate) fn is_null(&self) -> bool {
        match self {
            AST::Symbol(s) => s.name == "Null",
            _ => false,
        }
    }
    pub(crate) fn is_function(&self) -> bool {
        false
    }
    pub(crate) fn is_power(&self) -> bool {
        false
    }
    pub(crate) fn is_number(&self) -> bool {
        false
    }
    pub(crate) fn is_complex(&self) -> bool {
        false
    }
    pub(crate) fn is_integer(&self) -> bool {
        false
    }
    pub(crate) fn is_positive(&self) -> bool {
        false
    }
    pub(crate) fn is_negative(&self) -> bool {
        false
    }
}

#[allow(dead_code)]
impl Symbol {
    pub(crate) fn is_prefix(&self) -> bool {
        match self.kind {
            SymbolKind::Prefix(..) => true,
            _ => false,
        }
    }
    pub(crate) fn is_infix(&self) -> bool {
        match self.kind {
            SymbolKind::Infix(..) => true,
            _ => false,
        }
    }
    pub(crate) fn is_suffix(&self) -> bool {
        match self.kind {
            SymbolKind::Suffix(..) => true,
            _ => false,
        }
    }
    pub(crate) fn is_times(&self) -> bool {
        self.to_string() == "std::infix::times"
    }
}

pub(crate) fn is_unary(p: &[Parameter]) -> bool {
    if p.len() == 1 {
        if p[0].is_unary() {
            return true;
        }
    }
    false
}

pub(crate) fn is_multiary(p: &[Parameter]) -> bool {
    if p.len() == 1 {
        if p[0].is_multiary() {
            return true;
        }
    }
    false
}

impl Parameter {
    pub(crate) fn is_unary(&self) -> bool {
        if self.arguments.len() == 1 && self.options.len() == 0 {
            return true;
        }
        false
    }
    pub(crate) fn is_multiary(&self) -> bool {
        if self.arguments.len() > 1 && self.options.len() == 0 {
            return true;
        }
        false
    }
}
