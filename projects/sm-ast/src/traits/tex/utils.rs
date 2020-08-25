use crate::{
    ast::{Parameter, Symbol},
    traits::tex::BoxArea,
    ToTex,
};
use itertools::Itertools;

pub(crate) fn infix_tex(s: &Symbol, p: &Parameter) -> String {
    let lhs = &p.arguments[0];
    let rhs = &p.arguments[1];
    let ops = match s.name.as_str() {
        "plus" => "+",
        "times" => return format!("{} {}", lhs, rhs),
        _ => &s.name,
    };
    format!("{} {} {}", lhs, ops, rhs)
}

pub(crate) fn omit_brackets_function(p: &Parameter) -> String {
    let mut out = String::new();
    let args = &p.arguments;
    match args.len() {
        0 => out.push_str("()"),
        1 => {
            if args[0].width() <= 1 {
                out.push_str(&format!(" {}", args[0].to_tex()))
            }
            else {
                let max = p.height();
                if max > 1 {
                    out.push_str("\\left(");
                }
                else {
                    out.push_str("(");
                }
                out.push_str(&format!("{}", args[0].to_tex()));
                if max > 1 {
                    out.push_str("\\left)");
                }
                else {
                    out.push_str(")");
                }
            }
        }
        _ => {
            let max = p.height();
            if max > 1 {
                out.push_str("\\left(");
            }
            else {
                out.push_str("(");
            }
            let t = args.iter().map(|e| e.to_tex()).collect_vec();
            out.push_str(&format!("{}", t.join(", ")));
            if max > 1 {
                out.push_str("\\right)");
            }
            else {
                out.push_str(")");
            }
        }
    }
    return out;
}
