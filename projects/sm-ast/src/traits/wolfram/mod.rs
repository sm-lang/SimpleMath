use crate::{
    ast::{Expression, Symbol},
    AST,
};
use text_utils::capitalize_first_letter;
use wolfram_wxf::{ToWolfram, WolframValue};

impl ToWolfram for AST {
    fn to_wolfram(&self) -> WolframValue {
        match (*self).clone() {
            AST::EmptyStatement => WolframValue::Skip,
            //
            AST::Program(s) => WolframValue::function("CompoundExpression", s),
            //
            AST::Function(s, ps) => {
                let mut head = WolframValue::symbol(function_map(&s));
                for p in ps {
                    let mut vec = vec![];
                    for arg in p.arguments {
                        vec.push(arg.to_wolfram())
                    }
                    for (k, v) in p.options {
                        vec.push(WolframValue::function("Rule", vec![k.to_wolfram(), v.to_wolfram()]))
                    }
                    head = WolframValue::Function(Box::new(head), vec)
                }
                return head;
            }
            #[rustfmt::skip]
            AST::Boolean(b) => if b { WolframValue::symbol("True") } else { WolframValue::symbol("False") },
            AST::Integer(i) => WolframValue::BigInteger(i),
            AST::Decimal(f) => WolframValue::BigDecimal(Box::from(format!("{}", f))),
            AST::Symbol(s) => WolframValue::Symbol(Box::from(s.name)),
            AST::String(s) => WolframValue::String(Box::from(s)),
        }
    }
}

impl ToWolfram for Expression {
    fn to_wolfram(&self) -> WolframValue {
        unimplemented!()
    }
}

pub fn function_map(s: &Symbol) -> String {
    let name = s.name.as_str();
    match name {
        "factor" => "FactorInteger".to_string(),
        _ => capitalize_first_letter(name),
    }
}
