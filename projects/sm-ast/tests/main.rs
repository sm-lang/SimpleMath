use sm_ast::{evaluate::Runner, parser::ParserSettings, ToTex, ToWolfram, AST};

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
    let ast = parser.parse("[1, 2].head");
    println!("{}", &ast.to_wolfram_string());
    println!("{}", &ast.to_tex());

    let mut runner = Runner::from(ast);
    runner.forward();
    println!("{}", runner.ast.to_wolfram_string());
    println!("{}", runner.ast.to_tex());
}

#[test]
fn test_space_expression() {
    let parser = ParserSettings::default();
    let form = parser.parse("2 x y").to_wolfram_string();
    assert_eq!(form, "Times[2,x,y]");
    let form = parser.parse("2 -x y").to_wolfram_string();
    assert_eq!(form, "Subtract[2,Times[x,y]]");
}
