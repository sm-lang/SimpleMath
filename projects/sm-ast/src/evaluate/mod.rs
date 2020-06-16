use crate::AST;

mod rewrite;
mod utils;
mod forward;

pub struct Runner {
    pub ast: AST,
    ctx: Context,
}

pub struct Context {}
