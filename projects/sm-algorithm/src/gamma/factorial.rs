use num::{BigInt, Signed, ToPrimitive};

// ```wl
// TemplateApply[
// "cache.insert(`1`,BigInt::from(`2`));\n",
// {#, Fibonacci[#]}
// ]&
// %/@Join[Range[0,9],PowerRange[10,100,10]]
// %//StringJoin//CopyToClipboard
// ```


use num::{BigUint, One};
use crate::{Out, Error};

/// TODO:  Parallel Prime Swing Algorithm
///http://www.luschny.de/math/factorial/FastFactorialFunctions.htm
pub fn factorial_u(n: usize) -> BigUint {
    (1..=n).fold(BigUint::one(), |a, b| a * b)
}


pub fn factorial(n: &BigInt) -> Out<BigInt> {
    match n.to_isize() {
        Some(s) => {
            if s < 0 {
                Err(Error::ComplexInfinity)
            }
            else {
                Ok(BigInt::from(factorial_u(s as usize)))
            }
        }
        None => Err(Error::OverFlow),
    }
}


#[test]
fn factorial_u_test() {
    println!("{}", factorial(&BigInt::from(10000)).unwrap_or_default())
}
