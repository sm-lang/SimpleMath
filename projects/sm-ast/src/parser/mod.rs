use sm_parser::{Assoc, PrecClimber, Rule};

mod parse;
mod utils;

#[derive(Debug)]
pub struct ParserSettings {
    pub file: String,
    pub refine: bool,
}

pub const OPERATORS: &[(Rule, u32, Assoc); 5] = &[
    (Rule::Plus, 1, Assoc::Left),
    (Rule::Minus, 1, Assoc::Left),
    (Rule::Multiply, 2, Assoc::Left),
    (Rule::Divide, 2, Assoc::Left),
    (Rule::Power, 3, Assoc::Right),
];

pub static CLIMBER: PrecClimber<Rule> = PrecClimber::new_const(OPERATORS);
