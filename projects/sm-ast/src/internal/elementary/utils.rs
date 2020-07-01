use lazy_static::lazy_static;
use num::BigInt;
use std::collections::BTreeMap;

// ```wl
// TemplateApply[
// "cache.insert(`1`,BigInt::from(`2`));\n",
// {#, Fibonacci[#]}
// ]&
// %/@Join[Range[0,9],PowerRange[10,100,10]]
// %//StringJoin//CopyToClipboard
// ```
lazy_static! {
    static ref FIBONACCI_U_CACHE: BTreeMap<usize, BigInt> = {
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
        cache.insert(100, BigInt::from(354224848179261915075));
        return cache;
    };
}

#[test]
fn fibonacci_u() {
    println!("{:?}", FIBONACCI_U_CACHE.get(&0))
}

lazy_static! {
    static ref FACTORIAL_U_CACHE: BTreeMap<usize, BigInt> = {
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

        return cache;
    };
}

#[test]
fn factorial_u() {
    println!("{:?}", FACTORIAL_U_CACHE.get(&0))
}
