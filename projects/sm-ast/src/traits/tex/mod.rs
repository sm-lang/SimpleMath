mod symbol_map;

use crate::{ToTex, AST};
use symbol_map::*;

impl ToTex for AST {
    fn to_tex(&self) -> String {
        match (*self).clone() {
            AST::NewLine => format!("\n"),
            AST::Prefix(o, e) => match check_brackets(vec![&e]) {
                BracketType::None => format!("{}{}", prefix_map(&o), e.to_tex()),
                BracketType::Simple => format!("{}({})", prefix_map(&o), e.to_tex()),
                BracketType::Large => format!("{}\\left({}\\right)", prefix_map(&o), e.to_tex()),
            },
            AST::Suffix(o, e) => match check_brackets(vec![&e]) {
                BracketType::None => format!("{}{}", e.to_tex(), suffix_map(&o)),
                BracketType::Simple => format!("({}){}", e.to_tex(), suffix_map(&o)),
                BracketType::Large => format!("\\left({}\\right){}", prefix_map(&o), e.to_tex()),
            },
            AST::Binary(o, lhs, rhs) => match check_brackets(vec![&lhs, &rhs]) {
                BracketType::None => format!("{} {} {}", lhs.to_tex(), binary_map(&o), rhs.to_tex()),
                BracketType::Simple => format!("({} {} {})", lhs.to_tex(), binary_map(&o), rhs.to_tex()),
                BracketType::Large => format!("\\left({} {} {}\\right)", lhs.to_tex(), binary_map(&o), rhs.to_tex()),
            },
            AST::Integer { value, .. } => format!("{}", value),
            AST::Decimal(f) => format!("{}", f),
            AST::Symbol(s) => format!("{}", s.name),
            AST::String(s) => format!("\\text{{{}}}", s),
            _ => unimplemented!(),
        }
    }
}

pub enum BracketType {
    None,
    Simple,
    Large,
}

pub fn check_brackets(exprs: Vec<&Box<AST>>) -> BracketType {
    let mut v = vec![];
    for e in exprs {
        v.push(expression_height(&e))
    }
    match v.iter().max().unwrap() {
        0 => BracketType::None,
        1 => BracketType::Simple,
        _ => BracketType::Large,
    }
}

pub fn expression_height(e: &AST) -> usize {
    // TODO: compare ops
    //       (a + b) * c
    match e {
        AST::Function(_, _, _) => 1,
        AST::Prefix(_, _) => 1,
        AST::Suffix(_, _) => 1,
        AST::Binary(_, _, _) => 1,
        _ => 0,
    }
}
