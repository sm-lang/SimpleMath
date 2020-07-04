use crate::Output;
use num::{BigInt, BigUint, Integer, One, ToPrimitive, Zero};
use std::collections::BTreeMap;

pub fn fibonacci(n: &BigInt) -> Output {
    match n.to_isize() {
        Some(s) => {
            let u = fibonacci_fast_u(s as usize);
            if s < 0 && s.is_even() {
                Output::from(-u)
            }
            else {
                Output::from(u)
            }
        }
        None => Output::OverFlow,
    }
}

pub fn fibonacci_fold_u(n: usize) -> BigUint {
    (0..n).fold((BigUint::zero(), BigUint::one()), |x, _| (x.0.clone() + x.1, x.0)).0
}

pub fn fibonacci_fast_u(number: usize) -> BigUint {
    fn memoization(n: usize, m: &mut BTreeMap<usize, BigUint>) -> BigUint {
        match m.get(&n) {
            Some(s) => s.clone(),
            None => {
                let result = {
                    if n.is_even() {
                        let a = memoization(n / 2, m);
                        let b = memoization(n / 2 + 1, m);
                        a.clone() * (b * 2usize - a)
                    }
                    else {
                        let a = memoization(n / 2 + 1, m);
                        let b = memoization(n / 2, m);
                        a.clone() * a + b.clone() * b
                    }
                };
                m.insert(n, result.clone());
                result
            }
        }
    }
    let mut m = BTreeMap::new();
    m.insert(0usize, BigUint::from(0usize));
    m.insert(1usize, BigUint::from(1usize));
    m.insert(2usize, BigUint::from(1usize));
    memoization(number, &mut m)
}

#[test]
fn fibonacci_test() {
    assert_eq!(format!("{}", fibonacci_fold_u(100)), "354224848179261915075");
    assert_eq!(format!("{}", fibonacci_fast_u(100)), "354224848179261915075")
}
