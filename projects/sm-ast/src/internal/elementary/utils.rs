use num::{Zero, One, BigInt, ToPrimitive};
use num::BigUint;

fn fibonacci_add(n: i32, acc: BigUint, curr: BigUint) -> BigUint {
    if n <= 0 {
        acc
    }
    else{
        fibonacci_add(n - 1, &acc + curr, acc)
    }
}


pub fn fibonacci_int(n: &BigInt) -> BigInt {
    let u = fibonacci_add(n.to_i32().unwrap(), Zero::zero(), One::one());
    return BigInt::from(u)
}
