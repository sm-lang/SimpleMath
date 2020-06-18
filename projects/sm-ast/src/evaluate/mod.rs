use crate::AST;

mod rewrite;
mod utils;
#[allow(dead_code)]
mod forward;
mod traits;

#[derive(Debug, Clone)]
pub struct Runner {
    pub ast: AST,
    ctx: Context,
}

#[derive(Debug, Clone, Default)]
pub struct Context {}

