use sm_ast::{ToWolfram, AST};

#[test]
fn test() {
    let expr = AST::Suffix(Box::from("!"), Box::new(AST::Symbol(Box::from("a"))));
    println!("{}", expr.to_wolfram_string())
}
