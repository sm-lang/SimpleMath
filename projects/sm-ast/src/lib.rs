#![feature(option_unwrap_none)]

pub mod ast;
pub mod parser;
pub mod traits;
pub mod evaluate;
mod internal;
mod error;

pub use ast::AST;
pub use traits::{ToTex, ToWolfram};
pub use error::{SMError,SMResult};

#[test]
fn it_works() {}
