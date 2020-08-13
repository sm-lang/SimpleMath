use crate::wl_form;
use sm_ast::{parser::ParserSettings, ToWolfram};

#[test]
fn literal_number() {
    assert_eq!(wl_form("0"), "0");
    assert_eq!(wl_form("1"), "1");
    assert_eq!(wl_form("-1"), "Minus[1]");
    assert_eq!(wl_form("1.0"), "1.0");
    assert_eq!(wl_form("-1.0"), "Minus[1.0]");
    assert_eq!(wl_form("0xFF"), "255");
    assert_eq!(wl_form("0o77"), "63");
    assert_eq!(wl_form("0b11"), "3");
}

#[test]
fn literal_string() {
    assert_eq!(wl_form(r##########"           ""             "##########), "\"\"");
    assert_eq!(wl_form(r##########"          "  "            "##########), "\"  \"");
    assert_eq!(wl_form(r##########"        """  """          "##########), "\"  \"");
    assert_eq!(wl_form(r##########"       """ "" """         "##########), "\" \\\"\\\" \"");
    assert_eq!(wl_form(r##########"     r"""  \\  """        "##########), "\"  \\\\\\\\  \"");
}

#[test]
fn literal_repl() {
    assert_eq!(wl_form("¶"), "Input[-1]");
    assert_eq!(wl_form("¶1"), "Input[1]");
    assert_eq!(wl_form("¶¶"), "Input[-2]");
    assert_eq!(wl_form("¶¶¶"), "Input[-3]");
    assert_eq!(wl_form("⁋"), "Output[-1]");
    assert_eq!(wl_form("⁋1"), "Output[1]");
    assert_eq!(wl_form("⁋⁋"), "Output[-2]");
    assert_eq!(wl_form("⁋⁋⁋"), "Output[-3]");
}

#[test]
fn lambda_function() {
    assert_eq!(wl_form("#"), "Slot[]");
    assert_eq!(wl_form("#1"), "Slot[1]");
    assert_eq!(wl_form("#a"), "Slot[\"a\"]");
}

#[test]
fn literal_list() {
    assert_eq!(wl_form("[]"), "{}");
    assert_eq!(wl_form("[1]"), "{1}");
    assert_eq!(wl_form("[[]]"), "{{}}");
    assert_eq!(wl_form("[[1], 2]"), "{{1},2}");
}

#[test]
fn literal_list_advance() {
    let input = "[[1], 2, Nothing, Sequence(2, Sequence(3, 4))]";
    let parser = ParserSettings::default();
    let ast = parser.parse(input).unwrap();
    println!("{}", ast);
    println!("{}", ast.to_wolfram_string());
}
