#![feature(option_unwrap_none)]

mod ast;
mod error;
pub mod evaluate;

#[allow(dead_code)]
pub mod internal;
pub mod traits;
pub mod parser;

pub use ast::AST;
pub use error::{SMError, SMResult};
pub use traits::{ToTex, ToWolfram};
pub use evaluate::{Runner,Context};


#[test]
fn it_works() {}
