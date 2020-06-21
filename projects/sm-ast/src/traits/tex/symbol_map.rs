use crate::AST;
use std::{collections::BTreeMap, fmt::Write};

pub enum BracketType {
    None,
    Simple,
    Large,
}

pub fn binary_map(s: &str) -> Box<str> {
    let m = match s {
        "+-" => "\\mp",
        "-+" => "\\pm",
        _ => s,
    };
    Box::from(m)
}

pub fn function_map(s: &str, args: Vec<AST>, _kws: BTreeMap<AST, AST>) -> String {
    let mut out = String::new();
    match s {
        "sin" | "cos" => {
            write!(out, "\\\\{}", s);
            write!(out, "{}", omit_brackets_function(args));
        }
        _ => println!("Unknown function: {}", s),
    };
    return out;
}

fn omit_brackets_function(args: Vec<AST>) -> String {
    match check_brackets(&args) {
        BracketType::None => String::new(),
        BracketType::Simple => String::new(),
        BracketType::Large => String::new(),
    }
}

pub fn check_brackets(exprs: &Vec<AST>) -> BracketType {
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
    // TODO: compare ops.md
    //       (a + b) * c
    match e {
        AST::FunctionCall { .. } => 1,
        _ => 0,
    }
}
