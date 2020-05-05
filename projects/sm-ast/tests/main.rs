use sm_ast::{parser::ParserSettings, ToTex, ToWolfram, AST};

#[test]
fn test() {
    let a = AST::symbol("a");
    let expr = AST::Suffix(Box::from("!"), Box::new(a));
    println!("{}", expr.to_wolfram_string());
    println!("{}", expr.to_tex());
}

#[test]
fn test_parse() {
    let parser = ParserSettings::default();
    let ast = parser.parse("false");
    println!("{}", ast.to_wolfram_string());
    println!("{}", ast.to_tex());
}
