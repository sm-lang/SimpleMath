use crate::{ToWolfram, AST};

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

pub fn function_map(ast: &AST) -> Box<str> {
    let name = match ast {
        AST::Symbol(s) => s.name.as_str(),
        _ => return Box::from(ast.to_wolfram_string()),
    };
    let m = match name {
        "sin" => "Sin",
        "cos" => "Cos",
        "tan" => "Tan",
        "cot" => "Cot",
        _ => name,
    };
    Box::from(m)
}
