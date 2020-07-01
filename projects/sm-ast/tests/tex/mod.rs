use sm_ast::{parser::ParserSettings, ToTex};

#[test]
fn test_sin() {
    let parser = ParserSettings::default();
    let form = parser.parse("2 x y").unwrap().to_tex();
    assert_eq!(form, "Times[2,x,y]");
}
