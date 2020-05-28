use crate::{
    ast::{Position, Symbol},
    AST,
};
use std::{
    fmt,
    fmt::{Display, Formatter},
};

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

impl Display for AST {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            AST::Null => write!(f, "null"),
            AST::Boolean(b) => {
                if *b {
                    write!(f, "true")
                }
                else {
                    write!(f, "false")
                }
            }
            AST::Integer(n) => write!(f, "{}", n),
            AST::Decimal(_) => unimplemented!(),
            AST::Symbol(_) => unimplemented!(),
            AST::String(_) => unimplemented!(),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.name_space.len() == 0 {
            write!(f, "{}", self.name)
        }
        else {
            write!(f, "{}::{}", self.name_space.join("::"), self.name)
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.file)
    }
}
