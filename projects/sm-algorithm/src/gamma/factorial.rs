use num::{BigInt, ToPrimitive};

// ```wl
// TemplateApply[
// "cache.insert(`1`,BigInt::from(`2`));\n",
// {#, Fibonacci[#]}
// ]&
// %/@Join[Range[0,9],PowerRange[10,100,10]]
// %//StringJoin//CopyToClipboard
// ```

use crate::Output;
use num::{BigUint, One};

/// TODO:  Parallel Prime Swing Algorithm
/// http://www.luschny.de/math/factorial/FastFactorialFunctions.htm
pub fn factorial_u(n: usize) -> BigUint {
    (1..=n).fold(BigUint::one(), |a, b| a * b)
}

pub fn factorial(n: &BigInt) -> Output {
    match n.to_isize() {
        Some(s) => {
            if s < 0 {
                Output::ComplexInfinity
            }
            else {
                Output::from(factorial_u(s as usize))
            }
        }
        None => Output::OverFlow,
    }
}

#[test]
fn factorial_test() {
    println!("{}", factorial(&BigInt::from(100)))
}

pub fn factorial_mod() {
    unimplemented!()
}

pub fn factorial_digits() {
    unimplemented!()
}

pub fn factorial_tail_zero() {
    unimplemented!()
}
