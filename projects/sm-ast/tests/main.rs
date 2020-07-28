use sm_ast::{evaluate::Runner, parser::ParserSettings, ToTex, ToWolfram, AST};
mod parsed;
mod tex_form;

#[test]
fn test() {
    let a = AST::symbol("a");
    let expr = a;
    println!("{}", expr.to_wolfram_string());
    println!("{}", expr.to_tex());
}

#[test]
fn test_add() {
    let _engine = Runner::default();

    let parser = ParserSettings::default();
    let ast = parser.parse("0 + 1 + 2 x").unwrap();
    println!("{}", ast);
    println!("{}", ast.to_wolfram_string());

    let refined = ast.rewrite();
    println!("{}", refined);
    println!("{}", refined.to_wolfram_string());
}
