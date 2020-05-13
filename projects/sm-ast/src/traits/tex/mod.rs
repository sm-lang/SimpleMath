mod symbol_map;

use crate::{ToTex, AST};
use symbol_map::*;

impl ToTex for AST {
    fn to_tex(&self) -> String {
        match (*self).clone() {
            AST::EmptyStatement => format!(""),
            AST::NewLine => format!("\n"),
            AST::Null => format!("\\\\tt{{null}}"),
            AST::Expression { base, eos, .. } => {
                let s = if eos { ";" } else { "" };
                format!("{}{}", base.to_tex(), s)
            }
            AST::MultiplicativeExpression { expressions, .. } => {
                let e:Vec<_> = expressions.iter().map(AST::to_tex).collect();
                return e.join(" ")
            },
            AST::UnaryOperators { base, prefix, suffix, .. } => {
                let v = base.to_tex();
                let p = prefix.join(" ");
                let s = suffix.join(" ");
                format!("{}{}{}", p, v, s)
            }
            AST::InfixOperators { infix, lhs, rhs, .. } => {
                let l = lhs.to_tex();
                let r = rhs.to_tex();
                format!("{}{}{}", l, binary_map(&infix), r)
            }
            AST::Integer(i) => format!("{}", i),
            AST::Decimal(f) => format!("{}", f),
            AST::Symbol(s) => format!("{}", s.name),
            AST::String(s) => format!("\\text{{{}}}", s),

            AST::Program(_) => unimplemented!(),
            AST::FunctionCall { name: _, arguments: _, options: _, .. } => unimplemented!(),
            AST::Boolean(b) => {
                if b {
                    format!("\\\\tt{{true}}")
                }
                else {
                    format!("\\\\tt{{false}}")
                }
            }

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
        AST::FunctionCall { .. } => 1,
        _ => 0,
    }
}
