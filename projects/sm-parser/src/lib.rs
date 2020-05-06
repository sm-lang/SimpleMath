#[cfg(test)]
#[macro_use]
extern crate quote;
extern crate pest;
#[cfg(test)]
extern crate pest_generator;
#[cfg(test)]
extern crate proc_macro;

#[cfg(test)]
mod pre_build;

mod sm_parser;
pub use crate::sm_parser::{Rule, SMParser};
pub use pest::{
    iterators::Pair,
    prec_climber::{Assoc, Operator, PrecClimber},
    Parser, Span,
};

#[test]
fn it_works() {}
