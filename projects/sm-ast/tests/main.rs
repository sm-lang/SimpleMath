use sm_ast::{ToWolfram, AST, ToTex};

#[test]
fn test() {
    let expr = AST::Suffix(Box::from("!"), false, Box::new(AST::Symbol(Box::from("a"))));
    println!("{}", expr.to_wolfram_string());
    println!("{}", expr.to_tex());
}
