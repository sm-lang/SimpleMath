use crate::ast::{Symbol, SymbolKind};

pub fn prefix_map(text: &str) -> Symbol {
    let n = match text {
        "+" => "plus",
        "-" => "minus",
        "*" => "unpack",
        "!" => "not",
        _ => unreachable!(),
    };
    Symbol {
        name_space: vec![String::from("std"), String::from("prefix")],
        name: String::from(n),
        kind: SymbolKind::Prefix(Box::from(text)),
        attributes: 0,
    }
}

pub fn suffix_map(text: &str) -> Symbol {
    let n = match text {
        "!" => "factorial",
        "!!" => "factorial2",
        _ => unreachable!(),
    };
    Symbol {
        name_space: vec![String::from("std"), String::from("suffix")],
        name: String::from(n),
        kind: SymbolKind::Suffix(Box::from(text)),
        attributes: 0,
    }
}

pub fn infix_map(text: &str) -> Symbol {
    let (n, p) = match text {
        "+" => ("add", 80),
        "-" => ("subtract", 80),
        "*" => ("times", 140),
        "/" => ("divide", 140),
        "//" => ("quotient", 140),
        "^" => ("power", 150),
        _ => (text, 170),
    };
    Symbol {
        name_space: vec![String::from("std"), String::from("infix")],
        name: String::from(n),
        kind: SymbolKind::Infix(Box::from(text), p),
        attributes: 0,
    }
}
