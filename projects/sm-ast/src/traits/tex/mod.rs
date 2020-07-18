mod utils;
use crate::{
    traits::tex::utils::{binary_map, function_map},
    ToTex, AST,
};

impl ToTex for AST {
    fn to_tex(&self) -> String {
        match (*self).clone() {
            AST::EmptyStatement => format!(""),
            AST::NewLine => format!("\n"),
            AST::Null => format!(r"\\mathtt{{null}}"),
            AST::Expression { base, eos, .. } => {
                let s = if eos { ";" } else { "" };
                format!("{}{}", base.to_tex(), s)
            }
            AST::List(v) => {
                let max = v.iter().map(|e| e.height()).max().unwrap();
                let e: Vec<_> = v.iter().map(AST::to_tex).collect();
                if max > 1 { format!(r"\\left\\{{{}\\right\\}}", e.join(", ")) } else { format!(r"\\{{{}\\}}", e.join(", ")) }
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
            AST::String(s) => format!(r"\\text{{{}}}", s),

            AST::Program(_) => unimplemented!(),
            AST::FunctionCall { name, arguments, options, .. } => function_map(&name.to_tex(), arguments, options),
            AST::Boolean(b) => {
                if b {
                    format!(r"\\mathtt{{true}}")
                }
                else {
                    format!(r"\\mathtt{{false}}")
                }
            }
        }
    }
}

pub trait BoxArea {
    fn height(&self) -> usize {
        1
    }
    fn width(&self) -> usize {
        1
    }
}

impl BoxArea for AST {
    fn height(&self) -> usize {
        match self {
            AST::EmptyStatement => 0,
            AST::NewLine => 1,
            AST::Program(_) => 1,
            AST::Expression { .. } => 1,
            AST::FunctionCall { .. } => 1,
            AST::List(_) => 1,
            AST::UnaryOperators { .. } => 1,
            AST::InfixOperators { .. } => 1,
            _ => 1,
        }
    }
    fn width(&self) -> usize {
        match self {
            AST::EmptyStatement => 0,
            AST::NewLine => 1,
            AST::Program(_) => 1,
            AST::Expression { .. } => 1,
            AST::FunctionCall { .. } => 1,
            AST::List(v) => v.len(),
            AST::UnaryOperators { base, prefix, suffix, .. } => base.width() + prefix.len() + suffix.len(),
            AST::InfixOperators { infix: _, ref lhs, ref rhs, .. } => lhs.width() + rhs.width() + 1,
            _ => 1,
        }
    }
}
