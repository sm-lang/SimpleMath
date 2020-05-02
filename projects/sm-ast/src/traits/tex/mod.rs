mod symbol_map;

use crate::{ToTex, AST};
use symbol_map::*;

impl ToTex for AST {
    fn to_tex(&self) -> String {
        match (*self).clone() {
            AST::NewLine => format!("\n"),
            AST::Prefix(o, brackets, e) => {
                if brackets {
                    format!("{}({})", prefix_map(&o), e.to_tex())
                }
                else {
                    format!("{}{}", prefix_map(&o), e.to_tex())
                }
            }
            AST::Suffix(o, brackets, e) => {
                if brackets {
                    format!("({}){}", e.to_tex(), suffix_map(&o))
                }
                else {
                    format!("{}{}", e.to_tex(), suffix_map(&o))
                }
            }
            AST::Binary(o, brackets, lhs, rhs) => {
                if brackets {
                    format!("({} {} {})", lhs.to_tex(), binary_map(&o), rhs.to_tex())
                }
                else {
                    format!("{} {} {}", lhs.to_tex(), binary_map(&o), rhs.to_tex())
                }
            }
            AST::Integer(i) => format!("{}", i),
            AST::Decimal(f) => format!("{}", f),
            AST::Symbol(s) => format!("{}", s),
            AST::String(s) => format!("\\text{{{}}}", s),
            _ => unimplemented!(),
        }
    }
}
