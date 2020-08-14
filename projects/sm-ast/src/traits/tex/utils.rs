use crate::{traits::tex::BoxArea, ToTex, AST};
use itertools::Itertools;

pub(crate) fn binary_map(s: &str) -> String {
    let m = match s {
        "+-" => "\\mp",
        "-+" => "\\pm",
        "*" => return String::from(" "),
        _ => s,
    };
    format!(" {} ", m)
}

pub(crate) fn omit_brackets_function(args: &Vec<AST>) -> String {
    let mut out = String::new();
    match args.len() {
        0 => out.push_str("()"),
        1 => {
            if args[0].width() <= 1 {
                out.push_str(&format!(" {}", args[0].to_tex()))
            }
            else {
                if args[0].height() > 1 {
                    out.push_str("\\left(");
                }
                else {
                    out.push_str("(");
                }
                out.push_str(&format!("{}", args[0].to_tex()));
                if args[0].height() > 1 {
                    out.push_str("\\left)");
                }
                else {
                    out.push_str(")");
                }
            }
        }
        _ => {
            let max = args.iter().map(|e| e.height()).max().unwrap();
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
