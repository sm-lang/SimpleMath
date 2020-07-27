use crate::{
    ast::{Position, Symbol},
    internal, Context, AST,
};
use num::{traits::Pow, ToPrimitive};
use std::collections::BTreeMap;
// fn evaluate_list_omit(v: &mut Vec<AST>, ctx: &mut Context) -> Vec<AST> {
// let mut new = Vec::with_capacity(v.len());
// for e in v {
// e.rewrite();
// match e {
// AST::Symbol(ref s) => {
// if s.name == "Nothing" {
// continue;
// }
// else {
// new.push(e.clone())
// }
// }
// AST::Function(s,p) => match *name.clone() {
// AST::Symbol(s) => {
// if s.name == "Sequence" {
// let args = evaluate_list_omit(arguments, ctx);
// new.extend(args)
// }
// else {
// new.push(e.clone())
// }
// }
// _ => new.push(e.clone()),
// },
// _ => new.push(e.clone()),
// }
// }
// return new;
// }
fn evaluate_function(f: &Symbol, args: &Vec<AST>, _kws: &BTreeMap<AST, AST>, _position: Position, _: &mut Context) -> AST {
    match f.name.as_str() {
        "first" => internal::first(&args[0]).unwrap(),
        "last" => internal::last(&args[0]).unwrap(),
        "length" => internal::length(&args[0]).unwrap(),
        "factorial" => internal::factorial(&args[0]).unwrap(),
        "fibonacci" => internal::fibonacci(&args[0]).unwrap(),
        _ => {
            println!("{:?}", f);
            unimplemented!()
        }
    }
}

fn evaluate_additive(vec: &[AST], _: &mut Context) -> AST {
    match vec {
        [AST::Integer(lhs), AST::Integer(rhs)] => AST::Integer(lhs + rhs),
        _ => {
            println!("{:?}", vec);
            unimplemented!()
        }
    }
}

fn evaluate_multiplicative(vec: &[AST], _: &mut Context) -> AST {
    match vec {
        [AST::Integer(lhs), AST::Integer(rhs)] => AST::Integer(lhs * rhs),
        _ => {
            println!("{:?}", vec);
            unimplemented!()
        }
    }
}

fn evaluate_power(vec: &[AST], _: &mut Context) -> AST {
    match vec {
        [AST::Integer(lhs), AST::Integer(rhs)] => match rhs.to_u64() {
            None => AST::Integer(lhs.clone()),
            Some(s) => AST::Integer(lhs.pow(s)),
        },
        _ => {
            println!("{:?}", vec);
            unimplemented!()
        }
    }
}
