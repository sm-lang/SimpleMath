#[allow(unused_must_use)]
mod symbol_map;
mod utils;
use crate::{ToTex, AST};
use symbol_map::*;
pub use utils::BoxArea;

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
            AST::MultiplicativeExpression { terms: expressions, .. } => {
                let e: Vec<_> = expressions.iter().map(AST::to_tex).collect();
                e.join(" ")
            }
            AST::AdditiveExpression { terms: expressions, .. } => {
                let e: Vec<_> = expressions.iter().map(AST::to_tex).collect();
                e.join(" + ")
            }
            AST::List(v) => {
                // todo: height = 1
                let e: Vec<_> = v.iter().map(AST::to_tex).collect();
                format!("\\\\left\\\\{{{}\\\\right\\\\}}", e.join(", "))
            }

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
            AST::FunctionCall { name, arguments, options, .. } => function_map(&name.to_tex(), arguments, options),
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
