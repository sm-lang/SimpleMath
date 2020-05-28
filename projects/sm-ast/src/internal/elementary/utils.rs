use num::{BigInt, Zero};
use num::BigUint;
use cached::proc_macro::cached;

const ZERO: BigUint = BigUint::from(0usize);
const ONE: BigUint = BigUint::from(1usize);
const TWO: BigUint = BigUint::from(2usize);

//#[cached(size = 255)]
fn fibonacci_u(n: BigUint) -> BigUint {
    if n == Zero::zero(){

    }


    match n {
        ZERO | ONE => n,
        _ => fibonacci_u(n.clone() - ONE) + fibonacci_u(n - TWO)
    }
}

pub fn fibonacci_int(n: &BigInt) -> BigInt {
    BigInt::from(fibonacci_u(n.to_biguint().unwrap()))
}

//#[cached(size = 255)]
fn factorial_u(n: BigUint) -> BigUint {
    match n {
        ZERO | ONE => ONE,
        _ => n.clone() * factorial_u(n - ONE)
    }
}

pub fn factorial_int(n: &BigInt) -> BigInt {
    BigInt::from(factorial_u(n.to_biguint().unwrap()))
}