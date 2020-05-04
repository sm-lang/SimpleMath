use sm_ast::{ToTex, ToWolfram, AST};

#[test]
fn test() {
    let a = AST::symbol("a");
    let expr = AST::Suffix(Box::from("!"), Box::new(a));
    println!("{}", expr.to_wolfram_string());
    println!("{}", expr.to_tex());
}
