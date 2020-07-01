use crate::{traits::tex::BoxArea, ToTex, AST};
use itertools::Itertools;
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
    match s {
        "sin" | "cos" => format!("\\\\{}", omit_brackets_function(&args)),
        "arcsin" | "arccos" => format!("\\\\operatername{}", omit_brackets_function(&args)),
        _ => {
            println!("Unknown function: {}", s);
            format!("\\\\{:?}", args)
        }
    }
}

fn omit_brackets_function(args: &Vec<AST>) -> String {
    let mut out = String::new();
    match args.len() {
        0 => write!(out, "()")?,
        1 => write!(out, "{}", args[0].to_tex())?,
        _ => {
            // must use bracts
            let mut max = 1;
            for i in args {
                let h = i.height();
                if h > max {
                    max = h
                }
            }
            if max > 1 {
                write!(out, "\\left(")?;
            }
            else {
                write!(out, "(")?;
            }
            let t = args.iter().map(|e| e.to_tex()).collect_vec();
            write!(out, "{}", t.join(", "))?;
            if max > 1 {
                write!(out, "\\right)")?;
            }
            else {
                write!(out, ")")?;
            }
        }
    }
    return out;
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
    // TODO: compare operators.md
    //       (a + b) * c
    match e {
        AST::FunctionCall { .. } => 1,
        _ => 0,
    }
}
