// todo:
// https://mathematica.stackexchange.com/questions/80291/efficient-way-to-sum-all-the-primes-below-n-million-in-mathematica
// S(v,p)=S(v,p-1)-p\times(S(\left\lfloor\frac{v}{p}\right\rfloor,p-1)-S(p-1,p-1))

use crate::{
    error::Error::{Undefined, Unimplemented},
    Result,
};
use itertools::Itertools;
use num::{integer::Roots, BigInt, Signed, ToPrimitive};
use std::{collections::BTreeMap, str::FromStr};

#[rustfmt::skip]
pub fn prime_sum_u64(n: u64) -> u64 {
    let r = n.sqrt() as u64;
    assert!(r * r <= n && (r + 1).pow(2) > n);
    let mut v = (1..=r).map(|i| n / i).collect_vec();
    v.extend((0..*v.last().unwrap()).rev());
    let mut s: BTreeMap<u64, u64> = v.iter().copied().map(|i| (i, ((i + 1) * i / 2).wrapping_sub(1))).collect();
    for p in 2..=r {
        if s[&p] > s[&(p - 1)] {
            // p is prime
            let (sp, p2) = (s[&(p - 1)], p * p);
            for &ve in &v {
                if ve < p2 { break; }
                *s.get_mut(&ve).unwrap() -= p * (s[&(ve / p)] - sp);
            }
        }
    }
    return s[&n];
}

// https://oeis.org/A046731
// ```wl
// data=Import["https://oeis.org/A046731/b046731.txt","Table"]
// T="table.insert(BigInt::from_str(\"``\").unwrap(),BigInt::from_str(\"``\").unwrap());\n"
// StringJoin[TemplateApply[T,{10^#1,#2}]&@@@data]//CopyToClipboard
// ```
#[rustfmt::skip]
fn prime_sum_table() -> BTreeMap<BigInt, BigInt> {
    let mut table = BTreeMap::new();
    table.insert(BigInt::from_str("1").unwrap(), BigInt::from_str("0").unwrap());
    table.insert(BigInt::from_str("10").unwrap(), BigInt::from_str("17").unwrap());
    table.insert(BigInt::from_str("100").unwrap(), BigInt::from_str("1060").unwrap());
    table.insert(BigInt::from_str("1000").unwrap(), BigInt::from_str("76127").unwrap());
    table.insert(BigInt::from_str("10000").unwrap(), BigInt::from_str("5736396").unwrap());
    table.insert(BigInt::from_str("100000").unwrap(), BigInt::from_str("454396537").unwrap());
    table.insert(BigInt::from_str("1000000").unwrap(), BigInt::from_str("37550402023").unwrap());
    table.insert(BigInt::from_str("10000000").unwrap(), BigInt::from_str("3203324994356").unwrap());
    table.insert(BigInt::from_str("100000000").unwrap(), BigInt::from_str("279209790387276").unwrap());
    table.insert(BigInt::from_str("1000000000").unwrap(), BigInt::from_str("24739512092254535").unwrap());
    table.insert(BigInt::from_str("10000000000").unwrap(), BigInt::from_str("2220822432581729238").unwrap());
    // u128
    table.insert(BigInt::from_str("100000000000").unwrap(), BigInt::from_str("201467077743744681014").unwrap());
    table.insert(BigInt::from_str("1000000000000").unwrap(), BigInt::from_str("18435588552550705911377").unwrap());
    table.insert(BigInt::from_str("10000000000000").unwrap(), BigInt::from_str("1699246443377779418889494").unwrap());
    table.insert(BigInt::from_str("100000000000000").unwrap(), BigInt::from_str("157589260710736940541561021").unwrap());
    table.insert(BigInt::from_str("1000000000000000").unwrap(), BigInt::from_str("14692398516908006398225702366").unwrap());
    table.insert(BigInt::from_str("10000000000000000").unwrap(), BigInt::from_str("1376110854313351899159632866552").unwrap());
    table.insert(BigInt::from_str("100000000000000000").unwrap(), BigInt::from_str("129408626276669278966252031311350").unwrap());
    table.insert(BigInt::from_str("1000000000000000000").unwrap(), BigInt::from_str("12212914292949226570880576733896687").unwrap());
    table.insert(BigInt::from_str("10000000000000000000").unwrap(), BigInt::from_str("1156251260549368082781614413945980126").unwrap());
    table.insert(BigInt::from_str("100000000000000000000").unwrap(), BigInt::from_str("109778913483063648128485839045703833541").unwrap());
    table.insert(BigInt::from_str("1000000000000000000000").unwrap(), BigInt::from_str("10449550362130704786220283253063405651965").unwrap());
    table.insert(BigInt::from_str("10000000000000000000000").unwrap(), BigInt::from_str("996973504763259668279213971353794878368213").unwrap());
    table.insert(BigInt::from_str("100000000000000000000000").unwrap(), BigInt::from_str("95320530117168404458544684912403185555509650").unwrap());
    table.insert(BigInt::from_str("1000000000000000000000000").unwrap(), BigInt::from_str("9131187511156941634384410084928380134453142199").unwrap());
    table.insert(BigInt::from_str("10000000000000000000000000").unwrap(), BigInt::from_str("876268031750623105684911815303505535704119354853").unwrap());
    return table;
}

pub fn prime_sum_i(n: &BigInt) -> Result<BigInt> {
    if let Some(s) = prime_sum_table().get(&n) {
        return Ok(s.clone());
    };
    if n.is_negative() {
        return Err(Undefined(String::from("wrong def")));
    }
    return match n.to_u64() {
        None => Err(Unimplemented),
        Some(s) => Ok(BigInt::from(prime_sum_u64(s))),
    };
}

#[test]
fn test_prime_sum() {
    assert_eq!(format!("{}", prime_sum_u64(1)), "0");
    assert_eq!(format!("{}", prime_sum_u64(2)), "2");
    assert_eq!(format!("{}", prime_sum_u64(3)), "5");
    assert_eq!(format!("{}", prime_sum_u64(4)), "5");
    assert_eq!(format!("{}", prime_sum_u64(5)), "10");
    assert_eq!(format!("{}", prime_sum_u64(6)), "10");
    assert_eq!(format!("{}", prime_sum_u64(7)), "17");
    assert_eq!(format!("{}", prime_sum_u64(8)), "17");
    assert_eq!(format!("{}", prime_sum_u64(9)), "17");
    assert_eq!(format!("{}", prime_sum_u64(10)), "17");
    assert_eq!(format!("{}", prime_sum_u64(11)), "28");
    assert_eq!(format!("{}", prime_sum_u64(100)), "1060");
    assert_eq!(format!("{}", prime_sum_u64(1000)), "76127");
    assert_eq!(format!("{}", prime_sum_u64(10000)), "5736396");
}
