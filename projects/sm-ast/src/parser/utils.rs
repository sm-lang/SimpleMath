use crate::{ast::Position, parser::ParserSettings, AST};
use sm_parser::Span;
use std::collections::BTreeMap;

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

#[allow(dead_code)]
pub(crate) enum ApplyOrSlice {
    Apply(Vec<AST>, BTreeMap<AST, AST>),
    Slice,
}
