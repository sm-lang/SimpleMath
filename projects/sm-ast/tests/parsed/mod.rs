use sm_ast::{parser::ParserSettings, ToWolfram};

#[test]
fn literal_number() {
    let parser = ParserSettings::default();
    let wl_from = |s: &str| parser.parse(s).unwrap().to_wolfram_string();
    assert_eq!(wl_from("0"), "0");
    assert_eq!(wl_from("1"), "1");
    assert_eq!(wl_from("-1"), "Minus[1]");
    assert_eq!(wl_from("1.0"), "1.0");
    assert_eq!(wl_from("-1.0"), "Minus[1.0]");
}

#[test]
fn literal_string() {
    let parser = ParserSettings::default();
    let wl_from = |s: &str| parser.parse(s).unwrap().to_wolfram_string();
    assert_eq!(wl_from(r##########"           ""             "##########), "\"\"");
    assert_eq!(wl_from(r##########"          "  "            "##########), "\"  \"");
    assert_eq!(wl_from(r##########"        """  """          "##########), "\"  \"");
    assert_eq!(wl_from(r##########"       """ "" """         "##########), "\" \\\"\\\" \"");
    assert_eq!(wl_from(r##########"     r"""  \\  """        "##########), "\"  \\\\\\\\  \"");
}

#[test]
fn test_space_expression() {
    let parser = ParserSettings::default();
    let wl_from = |s: &str| parser.parse(s).unwrap().to_wolfram_string();
    assert_eq!(wl_from("2 x y"), "Times[2,x,y]");
    assert_eq!(wl_from("2 -x y"), "Subtract[2,Times[x,y]]");
    assert_eq!(wl_from("2*-x y"), "Times[2,Minus[Times[x,y]]]");
}
