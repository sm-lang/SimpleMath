#![feature(unchecked_math)]

mod cache;
mod combinatoric;
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
