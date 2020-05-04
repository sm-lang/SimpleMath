mod symbol_map;

use crate::AST;
use symbol_map::*;
use wolfram_wxf::{ToWolfram, WolframValue};

impl ToWolfram for AST {
    fn to_wolfram(&self) -> WolframValue {
        match (*self).clone() {
            AST::NewLine => WolframValue::Skip,
            //
            AST::Function(f, args, _kws) => {
                let mut v = vec![];
                for arg in args {
                    v.push(arg.to_wolfram())
                }
                WolframValue::Function(function_map(&f), v)

            },
            //
            AST::Prefix(o, expr) => WolframValue::Function(prefix_map(&o), vec![expr.to_wolfram()]),
            AST::Suffix(o, expr) => WolframValue::Function(suffix_map(&o), vec![expr.to_wolfram()]),
            AST::Binary(o, lhs, rhs) => WolframValue::Function(binary_map(&o), vec![lhs.to_wolfram(), rhs.to_wolfram()]),
            //
            AST::Null => WolframValue::new_symbol("None"),
            AST::Boolean(b) => {
                if b {
                    WolframValue::new_symbol("True")
                }
                else {
                    WolframValue::new_symbol("False")
                }
            }
            AST::Integer(i) => WolframValue::BigInteger(i),
            AST::Decimal(f) => WolframValue::BigDecimal(Box::from(format!("{}", f))),
            AST::Symbol(s) => WolframValue::Symbol(s.name),
            AST::String(s) => WolframValue::String(s),
        }
    }
}
