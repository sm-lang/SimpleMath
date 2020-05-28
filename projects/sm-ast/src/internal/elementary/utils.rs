use cached::proc_macro::cached;
use num::{BigInt, BigUint, One, Zero};

//#[cached(size = 255)]
fn fibonacci_u(n: BigUint) -> BigUint {
    if n == BigUint::zero() {
        n
    }
    else if n == BigUint::one() {
        n
    }
    else {
        fibonacci_u(n.clone() - BigUint::one()) + fibonacci_u(n - BigUint::one() - BigUint::one())
    }
}

pub fn fibonacci_int(n: &BigInt) -> BigInt {
    BigInt::from(fibonacci_u(n.to_biguint().unwrap()))
}

//#[cached(size = 255)]
fn factorial_u(n: BigUint) -> BigUint {
    if n == BigUint::zero() {
        BigUint::one()
    }
    else if n == BigUint::one() {
        BigUint::one()
    }
    else {
        n.clone() * factorial_u(n - BigUint::one())
    }
}

pub fn factorial_int(n: &BigInt) -> BigInt {
    BigInt::from(factorial_u(n.to_biguint().unwrap()))
}
