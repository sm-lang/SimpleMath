use crate::{ToWolfram, AST};
use text_utils::capitalize_first_letter;

pub fn prefix_map(s: &str) -> Box<str> {
    let m = match s {
        "+" => "Plus",
        "-" => "Minus",
        "!" => "Not",
        _ => s,
    };
    Box::from(m)
}

pub fn suffix_map(s: &str) -> Box<str> {
    let m = match s {
        "!" => "Factorial",
        "!!" => "Factorial2",
        _ => s,
    };
    Box::from(m)
}

pub fn binary_map(s: &str) -> Box<str> {
    let m = match s {
        "+" => "Plus",
        "-" => "Subtract",
        "*" => "Times",
        "/" => "Divide",
        "//" => "Quotient",
        _ => s,
    };
    Box::from(m)
}

pub fn function_map(ast: &AST) -> String {
    let name = match ast {
        AST::Symbol(s) => s.name.as_str(),
        _ => return ast.to_wolfram_string(),
    };
    match name {
        "factor" => "FactorInteger".to_string(),
        _ => capitalize_first_letter(name),
    }
}
