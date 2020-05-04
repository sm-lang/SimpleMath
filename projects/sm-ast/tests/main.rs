use sm_ast::{ast::Symbol, ToTex, ToWolfram, AST};

#[test]
fn test() {
    let a = AST::Symbol(Symbol { name: Box::from("a") });
    let expr = AST::Suffix(Box::from("!"), Box::new(a));
    println!("{}", expr.to_wolfram_string());
    println!("{}", expr.to_tex());
}
