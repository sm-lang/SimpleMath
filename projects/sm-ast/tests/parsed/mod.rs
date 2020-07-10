#[test]
fn test_space_expression() {
    let parser = ParserSettings::default();
    let form = parser.parse("2 x y").unwrap().to_wolfram_string();
    assert_eq!(form, "Times[2,x,y]");
    let form = parser.parse("2 -x y").unwrap().to_wolfram_string();
    assert_eq!(form, "Subtract[2,Times[x,y]]");
}