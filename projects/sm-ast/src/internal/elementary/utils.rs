use lazy_static::lazy_static;
use num::BigInt;
use std::collections::BTreeMap;
use std::sync::Mutex;

// ```wl
// TemplateApply[
// "cache.insert(`1`,BigInt::from(`2`));\n",
// {#, Fibonacci[#]}
// ]&
// %/@Join[Range[0,9],PowerRange[10,100,10]]
// %//StringJoin//CopyToClipboard
// ```
lazy_static! {
    pub static ref FIBONACCI_U_CACHE: Mutex<BTreeMap<usize, BigInt>> = {
        let mut cache = BTreeMap::new();
        cache.insert(0, BigInt::from(0));
        cache.insert(1, BigInt::from(1));
        cache.insert(2, BigInt::from(1));
        cache.insert(3, BigInt::from(2));
        cache.insert(4, BigInt::from(3));
        cache.insert(5, BigInt::from(5));
        cache.insert(6, BigInt::from(8));
        cache.insert(7, BigInt::from(13));
        cache.insert(8, BigInt::from(21));
        cache.insert(9, BigInt::from(34));
        cache.insert(10, BigInt::from(55));
        cache.insert(100, BigInt::from(354224848179261915075u128));
        return Mutex::new(cache)
    };
}


fn fibonacci_u(u:usize)->BigInt {
    let mut cache = FIBONACCI_U_CACHE.lock().unwrap();
    match cache.get(&u) {
        Some(s) => {
            return s.clone()
        },
        None => {
            let new = fibonacci_u(u-1) + fibonacci_u(u-2);
            cache.insert(u,new).unwrap()
        },
    }
}

#[test]
fn fibonacci_u_test() {
    println!("{}", fibonacci_u(100))
}

lazy_static! {
    pub static ref FACTORIAL_U_CACHE: Mutex<BTreeMap<usize, BigInt>> = {
        let mut cache = BTreeMap::new();
        cache.insert(0, BigInt::from(1));
        cache.insert(1, BigInt::from(1));
        cache.insert(2, BigInt::from(2));
        cache.insert(3, BigInt::from(6));
        cache.insert(4, BigInt::from(24));
        cache.insert(5, BigInt::from(120));
        cache.insert(6, BigInt::from(720));
        cache.insert(7, BigInt::from(5040));
        cache.insert(8, BigInt::from(40320));
        cache.insert(9, BigInt::from(362880));
        cache.insert(10, BigInt::from(3628800));
        return Mutex::new(cache)
    };
}

fn factorial_u(u:usize)->BigInt {
    let mut cache = FIBONACCI_U_CACHE.lock().unwrap();
    match cache.get(&u) {
        Some(s) => {
            return s.clone()
        },
        None => {
            let new = BigInt::from(u) * factorial_u(u-1);
            cache.insert(u,new).unwrap()
        },
    }
}

#[test]
fn factorial_u_test() {
    println!("{}", factorial_u(1000))
}
