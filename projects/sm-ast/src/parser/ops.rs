use crate::ast::Symbol;

pub fn prefix_map(s: &str) -> Symbol {
    let m = match s {
        "+" => "plus",
        "-" => "minus",
        "*" => "unpack",
        _ => unreachable!(),
    };
    Symbol::from(String::from("std::prefix::") + m)
}

pub fn suffix_map(s: &str) -> Symbol {
    let m = match s {
        "!" => "factorial",
        "!!" => "factorial2",
        _ => unreachable!(),
    };
    Symbol::from(String::from("std::suffix::") + m)
}
