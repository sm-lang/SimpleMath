use crate::parser::ParserSettings;

mod rewrite;
mod utils;
#[allow(dead_code)]
mod forward;
mod traits;

#[derive(Debug, Clone)]
pub struct Runner {
    parser: ParserSettings,
    ctx: Context,
}

#[derive(Debug, Clone, Default)]
pub struct Context {}

