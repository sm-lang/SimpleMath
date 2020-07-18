use crate::{
    error::Error::{Undefined, Unimplemented},
    Result,
};
use num::{BigInt, Signed, ToPrimitive};
use std::{collections::BTreeMap, str::FromStr};

// https://oeis.org/A006880
// ```wl
// data=Import["https://oeis.org/A006880/b006880.txt","Table"]
// T="table.insert(BigInt::from_str(\"``\").unwrap(),BigInt::from_str(\"``\").unwrap());\n"
// StringJoin[TemplateApply[T,{10^#1,#2}]&@@@data]//CopyToClipboard
// ```
#[rustfmt::skip]
fn prime_count_table() -> BTreeMap<BigInt, BigInt> {
    let mut table = BTreeMap::new();
    table.insert(BigInt::from_str("1").unwrap(), BigInt::from_str("0").unwrap());
    table.insert(BigInt::from_str("10").unwrap(), BigInt::from_str("4").unwrap());
    table.insert(BigInt::from_str("100").unwrap(), BigInt::from_str("25").unwrap());
    table.insert(BigInt::from_str("1000").unwrap(), BigInt::from_str("168").unwrap());
    table.insert(BigInt::from_str("10000").unwrap(), BigInt::from_str("1229").unwrap());
    table.insert(BigInt::from_str("100000").unwrap(), BigInt::from_str("9592").unwrap());
    table.insert(BigInt::from_str("1000000").unwrap(), BigInt::from_str("78498").unwrap());
    table.insert(BigInt::from_str("10000000").unwrap(), BigInt::from_str("664579").unwrap());
    table.insert(BigInt::from_str("100000000").unwrap(), BigInt::from_str("5761455").unwrap());
    table.insert(BigInt::from_str("1000000000").unwrap(), BigInt::from_str("50847534").unwrap());
    table.insert(BigInt::from_str("10000000000").unwrap(), BigInt::from_str("455052511").unwrap());
    table.insert(BigInt::from_str("100000000000").unwrap(), BigInt::from_str("4118054813").unwrap());
    table.insert(BigInt::from_str("1000000000000").unwrap(), BigInt::from_str("37607912018").unwrap());
    table.insert(BigInt::from_str("10000000000000").unwrap(), BigInt::from_str("346065536839").unwrap());
    table.insert(BigInt::from_str("100000000000000").unwrap(), BigInt::from_str("3204941750802").unwrap());
    table.insert(BigInt::from_str("1000000000000000").unwrap(), BigInt::from_str("29844570422669").unwrap());
    table.insert(BigInt::from_str("10000000000000000").unwrap(), BigInt::from_str("279238341033925").unwrap());
    table.insert(BigInt::from_str("100000000000000000").unwrap(), BigInt::from_str("2623557157654233").unwrap());
    table.insert(BigInt::from_str("1000000000000000000").unwrap(), BigInt::from_str("24739954287740860").unwrap());
    table.insert(BigInt::from_str("10000000000000000000").unwrap(), BigInt::from_str("234057667276344607").unwrap());
    table.insert(BigInt::from_str("100000000000000000000").unwrap(), BigInt::from_str("2220819602560918840").unwrap());
    table.insert(BigInt::from_str("1000000000000000000000").unwrap(), BigInt::from_str("21127269486018731928").unwrap());
    table.insert(BigInt::from_str("10000000000000000000000").unwrap(), BigInt::from_str("201467286689315906290").unwrap());
    table.insert(BigInt::from_str("100000000000000000000000").unwrap(), BigInt::from_str("1925320391606803968923").unwrap());
    table.insert(BigInt::from_str("1000000000000000000000000").unwrap(), BigInt::from_str("18435599767349200867866").unwrap());
    table.insert(BigInt::from_str("10000000000000000000000000").unwrap(), BigInt::from_str("176846309399143769411680").unwrap());
    table.insert(BigInt::from_str("100000000000000000000000000").unwrap(), BigInt::from_str("1699246750872437141327603").unwrap());
    table.insert(BigInt::from_str("1000000000000000000000000000").unwrap(), BigInt::from_str("16352460426841680446427399").unwrap());
    return table;
}

pub fn prime_count_i(n: &BigInt) -> Result<BigInt> {
    if let Some(s) = prime_count_table().get(&n) {
        return Ok(s.clone());
    };
    if n.is_negative() {
        return Err(Undefined(String::from("wrong def")));
    }
    match n.to_u64() {
        None => Err(Unimplemented),
        Some(_) => Err(Unimplemented),
    }
}
