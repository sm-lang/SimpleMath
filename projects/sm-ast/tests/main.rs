use sm_ast::{parser::ParserSettings, ToTex, ToWolfram, AST};

#[test]
fn test() {
    let a = AST::symbol("a");
    let expr = a;
    println!("{}", expr.to_wolfram_string());
    println!("{}", expr.to_tex());
}

#[test]
fn test_parse() {
    let parser = ParserSettings::default();
    let ast = parser.parse("sin(in:2,3)");
    println!("{}", ast.to_wolfram_string());
    println!("{}", ast.to_tex());
}
