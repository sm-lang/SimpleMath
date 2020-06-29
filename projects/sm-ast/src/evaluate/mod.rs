use crate::parser::ParserSettings;

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
pub struct Context {}
