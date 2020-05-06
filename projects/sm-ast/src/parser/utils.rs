use crate::{ast::Position, parser::ParserSettings};
use sm_parser::Span;

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
