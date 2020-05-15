mod symbol_map;

use crate::AST;
use symbol_map::*;
use wolfram_wxf::{ToWolfram, WolframValue};

impl ToWolfram for AST {
    fn to_wolfram(&self) -> WolframValue {
        match (*self).clone() {
            AST::EmptyStatement | AST::NewLine => WolframValue::Skip,
            //
            AST::Program(s) => {
                let v: Vec<_> = s.iter().map(|s| s.to_wolfram()).collect();
                WolframValue::Function(Box::from("CompoundExpression"), v)
            }
            AST::Expression { base, .. } => base.to_wolfram(),
            AST::MultiplicativeExpression { expressions: e, .. } => {
                WolframValue::Function(Box::from("Times"), e.iter().map(AST::to_wolfram).collect())
            }
            //
            AST::FunctionCall { name, arguments, options, .. } => {
                let mut vec = vec![];
                for arg in arguments {
                    vec.push(arg.to_wolfram())
                }
                for (k, v) in options {
                    vec.push(WolframValue::Function(Box::from("Rule"), vec![k.to_wolfram(), v.to_wolfram()]))
                }
                WolframValue::Function(Box::from(function_map(&name)), vec)
            }
            AST::List(v) => {
                let v: Vec<_> =v.iter().map(|s| s.to_wolfram()).collect();
                WolframValue::Function(Box::from("List"), v)
            }
            //
            AST::UnaryOperators { base, prefix, suffix, .. } => {
                let mut v = base.to_wolfram();
                for o in suffix {
                    v = WolframValue::Function(suffix_map(&o), vec![v])
                }
                for o in prefix {
                    v = WolframValue::Function(prefix_map(&o), vec![v])
                }
                return v;
            }
            AST::InfixOperators { infix, lhs, rhs, .. } => {
                WolframValue::Function(binary_map(&infix), vec![lhs.to_wolfram(), rhs.to_wolfram()])
            }
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
            AST::Symbol(s) => WolframValue::Symbol(Box::from(s.name)),
            AST::String(s) => WolframValue::String(Box::from(s)),
        }
    }
}
