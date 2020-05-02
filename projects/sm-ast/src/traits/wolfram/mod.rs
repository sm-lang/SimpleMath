mod symbol_map;

use crate::AST;
use symbol_map::*;
use wolfram_wxf::{ToWolfram, WolframValue};

impl ToWolfram for AST {
    fn to_wolfram(&self) -> WolframValue {
        match (*self).clone() {
            AST::NewLine => WolframValue::Skip,
            AST::Prefix(o, _, expr) => WolframValue::Function(prefix_map(&o), vec![expr.to_wolfram()]),
            AST::Suffix(o, _, expr) => WolframValue::Function(suffix_map(&o), vec![expr.to_wolfram()]),
            AST::Binary(o, _, lhs, rhs) => WolframValue::Function(binary_map(&o), vec![lhs.to_wolfram(), rhs.to_wolfram()]),
            AST::Integer(i) => WolframValue::BigInteger(i),
            AST::Decimal(f) => WolframValue::BigDecimal(Box::from(format!("{}", f))),
            AST::Symbol(s) => WolframValue::Symbol(s),
            AST::String(s) => WolframValue::String(s),
        }
    }
}
