use crate::{
    error::Error::{Indeterminate, OverFlow},
    Result,
};
use num::{
    bigint::Sign,
    integer::{gcd, ExtendedGcd},
    traits::Pow,
    BigInt, BigUint, Integer, One, ToPrimitive, Zero,
};

pub fn power_iu(a: &BigInt, b: &BigUint) -> Result<BigInt> {
    if a.is_zero() && b.is_zero() {
        return Err(Indeterminate);
    }
    else if a.is_zero() || b.is_zero() {
        return Ok(BigInt::zero());
    }
    else if a.is_one() {
        return Ok(BigInt::one());
    }
    else if b.is_one() {
        return Ok(BigInt::from_biguint(Sign::Plus, b.clone()));
    }
    match b.to_u32() {
        None => Err(OverFlow),
        Some(u) => Ok(a.pow(u)),
    }
}

#[test]
fn test_power() {
    assert_eq!(BigInt::from(-2).pow(16u64), BigInt::from(65536));
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(BigInt::from(15), BigInt::from(21)), BigInt::from(3));
}

pub fn extended_gcd2(a: &BigInt, b: &BigInt) -> (BigInt, Vec<BigInt>) {
    let gcd = a.extended_gcd(b);
    (gcd.gcd, vec![gcd.x, gcd.y])
}

pub fn extended_gcd_n(v: &[BigInt]) -> (BigInt, Vec<BigInt>) {
    println!("{:?}", v);
    unimplemented!()
}

#[test]
fn test_extend_gcd() {
    let ou = extended_gcd2(&BigInt::from(314), &BigInt::from(271));
    println!("{:?}", ou)
}

/// inverse in modulo
/// $ax \equiv 1 \pmod{m}$
pub fn modulo_inverse(a: &BigInt, m: &BigInt) -> Option<BigInt> {
    let ExtendedGcd { gcd, x, .. } = a.extended_gcd(m);
    if gcd.is_one() { Some(x + m) } else { None }
}

#[test]
fn test_modulo_inverse() {
    let ou = modulo_inverse(&BigInt::from(-5), &BigInt::from(7));
    println!("{:?}", ou)
}

/// division in modulo
/// $bx \equiv a \pmod{m}$
pub fn modulo_division(a: &BigInt, b: &BigInt, m: &BigInt) -> Option<BigInt> {
    let ExtendedGcd { gcd, x: _, y, .. } = b.extended_gcd(m);
    if (a % &gcd).is_zero() { Some(a / &gcd * y) } else { None }
}

#[test]
fn test_modulo_division() {
    let ou = modulo_division(&BigInt::from(42), &BigInt::from(32), &BigInt::from(98));
    println!("{:?}", ou)
}

/// $`\gcd(x, y) = 1`$
pub fn is_coprime(x: BigInt, y: BigInt) -> bool {
    gcd(x, y).is_one()
}

/// Chinese remainder theorem
pub fn chinese_remainder(u: &[BigInt], m: &[BigInt]) -> Option<BigInt> {
    if u.len() != m.len() {
        return None;
    }
    let mut v = Vec::with_capacity(u.len());
    for (i, (u_i, m_i)) in u.iter().zip(m.iter()).enumerate() {
        let c_i = modulo_inverse(&m[0..i].iter().fold(BigInt::one(), |p, v| p * v % m_i), &m_i.clone())?;
        let t = v.iter().zip(m.iter()).rev().fold(BigInt::zero(), |t, (v_j, m_j)| m_j * t + v_j % m_i);
        v.push((u_i - t) * c_i % m_i);
    }
    let mut ret = v.pop().unwrap();
    for (v_i, m_i) in v.iter().zip(m.iter()).rev() {
        ret = ret * m_i + v_i;
    }
    return Some(ret);
}

#[test]
fn test_crt() {
    let u = vec![BigInt::from(2), BigInt::from(3), BigInt::from(2)];
    let m = vec![BigInt::from(3), BigInt::from(5), BigInt::from(7)];
    let a = chinese_remainder(&u, &m).unwrap();
    println!("{:?}", a)
}

pub fn chinese_remainder_d(u: &[BigInt], m: &[BigInt], d: BigInt) -> Option<BigInt> {
    println!("{:?}\n{:?}\n{}", u, m, d);
    unimplemented!()
}
