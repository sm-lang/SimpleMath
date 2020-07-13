// todo:
// https://mathematica.stackexchange.com/questions/80291/efficient-way-to-sum-all-the-primes-below-n-million-in-mathematica
// S(v,p)=S(v,p-1)-p\times(S(\left\lfloor\frac{v}{p}\right\rfloor,p-1)-S(p-1,p-1))

use num::{integer::Roots, BigUint};
use primal::is_prime;
use std::collections::BTreeMap;

fn prime_sum_s(i: usize, j: usize) -> BigUint {
    fn memoization(v: usize, p: usize, m: &mut BTreeMap<(usize, usize), BigUint>) -> BigUint {
        if v == 1 {
            BigUint::from(0usize)
        }
        else if v == 2 {
            BigUint::from(2usize)
        }
        else if p == 1 {
            BigUint::from((2 + v) * (v - 1) / 2)
        }
        else if p.pow(2) <= v && is_prime(p as u64) {
            match m.get(&(v, p)) {
                Some(s) => s.clone(),
                None => {
                    let a = memoization(v, p - 1, m);
                    let b = memoization(v / p, p - 1, m);
                    let c = memoization(p - 1, p - 1, m);
                    let result = a - (b - c)*p;
                    m.insert((v, p), result.clone());
                    result
                }
            }
        }
        else {
            match m.get(&(v, p)) {
                Some(s) => s.clone(),
                None => {
                    let result = memoization(v, p - 1, m);
                    m.insert((v, p), result.clone());
                    result
                }
            }
        }
    }
    memoization(i, j, &mut BTreeMap::new())
}

#[test]
fn test() {
    println!("{}", prime_sum_s(10000000, 10000000.sqrt()));
}
