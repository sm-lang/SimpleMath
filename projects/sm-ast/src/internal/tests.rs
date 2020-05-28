use crate::{internal::{fibonacci,factorial}, AST};

#[test]
fn fibonacci_int() {
    let input = AST::integer(0);
    let output = AST::integer(0);
    assert_eq!(fibonacci(&input).unwrap(), output);

    let input = AST::integer(100u128);
    let output = AST::integer(354224848179261915075u128);
    assert_eq!(fibonacci(&input).unwrap(), output);
}

#[test]
fn factorial_int() {
    let input = AST::integer(0);
    let output = AST::integer(1);
    assert_eq!(factorial(&input).unwrap(), output);

    let input = AST::integer(10u128);
    let output = AST::integer(3628800u128);
    assert_eq!(factorial(&input).unwrap(), output);
}
