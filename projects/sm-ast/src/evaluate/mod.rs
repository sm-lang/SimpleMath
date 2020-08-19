use crate::{ast::Symbol, parser::ParserSettings, AST};
use std::collections::{BTreeMap, BTreeSet};

mod forward;
mod rewrite;
mod traits;
mod utils;

#[derive(Debug, Clone)]
pub struct Runner {
    parser: ParserSettings,
    ctx: Context,
}

#[derive(Debug, Clone, Default)]
pub struct Context {
    index: usize,
    inputs: BTreeMap<usize, String>,
    outputs: BTreeMap<usize, AST>,
    // is this ok?
    symbols: BTreeSet<Symbol>,
}
