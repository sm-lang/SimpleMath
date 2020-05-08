pub fn binary_map(s: &str) -> Box<str> {
    let m = match s {
        "+-" => "\\mp",
        "-+" => "\\pm",
        _ => s,
    };
    Box::from(m)
}

pub fn function_map(s: &str) -> Box<str> {
    let m = match s {
        _ => s,
    };
    Box::from(m)
}
