use num::{BigUint, One, Zero};

// TODO: Θ(logn)
// https://www.nayuki.io/page/fast-fibonacci-algorithms
pub fn fibonacci_u(n: usize) -> BigUint {
    (0..n).fold((BigUint::zero(), BigUint::one()), |x, _| (x.0.clone() + x.1, x.0)).0
}

#[test]
fn fibonacci_u_test() {
    println!("{}", fibonacci_u(100))
}

