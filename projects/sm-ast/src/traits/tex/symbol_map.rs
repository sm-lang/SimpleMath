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

pub fn function_map(s: &str) -> Box<str> {
    let m = match s {
        "sin" => "Sin",
        "cos" => "Cos",
        "tan" => "Tan",
        "cot" => "Cot",
        _ => s,
    };
    Box::from(m)
}
