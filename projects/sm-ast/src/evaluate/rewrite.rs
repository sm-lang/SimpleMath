use crate::{
    ast::{Position, Symbol},
    evaluate::Context,
    internal, ToTex, ToWolfram, AST,
};
use num::{traits::Pow, ToPrimitive};
use std::collections::BTreeMap;
use crate::evaluate::utils::is_container;

// todo: remove redundant forward
impl AST {
    pub fn rewrite(&mut self, ctx: &mut Context) {
        match self {
            AST::Expression { base, .. } => base.rewrite(ctx),
            AST::FunctionCall { name, ref arguments, ref options, ref position } => {
                name.rewrite(ctx);
                match **name {
                    AST::Symbol(ref s) => {
                        if !is_container(&s.name) {
                            *self = evaluate_function(s, arguments, options, position.clone(), ctx)
                        }
                    }
                    _ => unimplemented!(),
                }
            }
            AST::MultiplicativeExpression { expressions, .. } => {
                *self = evaluate_multiplicative(&expressions, ctx);
            }
            AST::List(v) => {
                *self = AST::List(evaluate_list_omit(v, ctx));
            }
            AST::UnaryOperators { .. } => {}
            AST::InfixOperators { infix, lhs, rhs, .. } => {
                lhs.rewrite(ctx);
                rhs.rewrite(ctx);
                // ,  `Vec<&mut Box<AST>>` ->  `&mut [AST]`
                match infix.as_str() {
                    "+" => {
                        *self = evaluate_additive(&vec![*lhs.clone(), *rhs.clone()], ctx);
                    }
                    "*" | "×" => {
                        *self = evaluate_multiplicative(&vec![*lhs.clone(), *rhs.clone()], ctx);
                    }
                    "^" => {
                        *self = evaluate_power(&vec![*lhs.clone(), *rhs.clone()], ctx);
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
}

fn evaluate_list_omit(v: &mut Vec<AST>, ctx: &mut Context) -> Vec<AST> {
    let mut new = Vec::with_capacity(v.len());
    for e in v {
        e.rewrite(ctx);
        match e {
            AST::Symbol(ref s) => {
                if s.name == "Nothing" {
                    continue;
                }
                else {
                    new.push(e.clone())
                }
            }
            AST::FunctionCall { name, arguments, .. } => match *name.clone() {
                AST::Symbol(s) => {
                    if s.name == "Sequence" {
                        let args = evaluate_list_omit(arguments, ctx);
                        new.extend(args)
                    }
                    else {
                        new.push(e.clone())
                    }
                }
                _ => new.push(e.clone()),
            },
            _ => new.push(e.clone()),
        }
    }
    return new;
}

fn evaluate_function(f: &Symbol, args: &Vec<AST>, _kws: &BTreeMap<AST, AST>, _position: Position, _: &mut Context) -> AST {
    match f.name.as_str() {
        "wolfram_form" => AST::string(&args[0].to_wolfram_string()),
        "tex_form" => AST::string(&args[0].to_tex()),
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
