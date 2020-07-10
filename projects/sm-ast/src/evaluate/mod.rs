use crate::{ast::Symbol, parser::ParserSettings, AST};
use std::collections::{BTreeSet, HashMap};

#[allow(dead_code)]
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
    inputs: HashMap<usize, AST>,
    outputs: HashMap<usize, AST>,
    // is this ok?
    symbols: BTreeSet<Symbol>,
}
