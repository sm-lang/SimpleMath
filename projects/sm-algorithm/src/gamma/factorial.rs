use crate::Result;
use num::{BigInt, ToPrimitive};
// ```wl
// TemplateApply[
// "cache.insert(`1`,BigInt::from(`2`));\n",
// {#, Fibonacci[#]}
// ]&
// %/@Join[Range[0,9],PowerRange[10,100,10]]
// %//StringJoin//CopyToClipboard
// ```

use crate::Error::{ComplexInfinity, OverFlow};
use num::{BigUint, One};

/// TODO:  Parallel Prime Swing Algorithm
/// http://www.luschny.de/math/factorial/FastFactorialFunctions.htm
/// http://www.luschny.de/math/factorial/SwingFactorialSagePython.html
pub fn factorial_fold_u(n: usize) -> BigUint {
    (1..=n).fold(BigUint::one(), |a, b| a * b)
}

pub fn factorial_i(n: &BigInt) -> Result<BigInt> {
    match n.to_isize() {
        Some(s) => {
            if s < 0 {
                Err(ComplexInfinity)
            }
            else {
                Ok(BigInt::from(factorial_fold_u(s as usize)))
            }
        }
        None => Err(OverFlow),
    }
}

#[test]
fn factorial_test() {
    println!("{}", factorial_i(&BigInt::from(100)).unwrap())
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
