use crate::{
    ast::{Position, Symbol},
    parser::ParserSettings,
    AST,
};
use sm_parser::Span;
use std::{
    collections::BTreeMap,
    fmt,
    fmt::{Debug, Formatter},
};

impl Default for ParserSettings {
    fn default() -> Self {
        Self { file: String::from("anonymous"), refine: true }
    }
}

impl ParserSettings {
    pub(crate) fn get_position(&self, s: Span) -> Position {
        Position { file: self.file.clone(), start: s.start_pos().line_col(), end: s.end_pos().line_col() }
    }
}

pub(crate) enum ApplyOrSlice {
    Apply(Vec<AST>, BTreeMap<AST, AST>),
    Slice,
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.name_space.len() == 0 {
            write!(f, "{}", self.name)
        }
        else {
            write!(f, "{}::{}", self.name_space.join("::"), self.name)
        }
    }
}
