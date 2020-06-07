use crate::AST;

mod context;
mod utils;
pub use utils::{check_symbol_alias};

pub struct Runner {
    pub ast: AST,
    ctx: Context,
}

pub struct Context {}
