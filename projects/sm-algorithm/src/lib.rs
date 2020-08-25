#![feature(unchecked_math)]

mod basic;
mod cache;
mod combinatorics;
mod digits;
mod error;
mod gamma;
mod linear;
mod prime;

pub use cache::{Cache, GlobalCache};
pub use error::{Error, Result};
pub use gamma::*;
pub use linear::*;
pub use prime::*;
pub use basic::*;