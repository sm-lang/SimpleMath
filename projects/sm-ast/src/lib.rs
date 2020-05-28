#![feature(option_unwrap_none)]

pub mod ast;
mod error;
pub mod evaluate;
#[allow(dead_code)]
mod internal;
pub mod parser;
pub mod traits;

pub use ast::AST;
pub use error::{SMError, SMResult};
pub use traits::{ToTex, ToWolfram};

#[test]
fn it_works() {}
