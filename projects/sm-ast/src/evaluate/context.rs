use crate::{evaluate::Context, AST, ToWolfram, ToTex};
use num::{traits::Pow, ToPrimitive};
use std::collections::BTreeMap;
use crate::ast::Symbol;
use crate::internal;

impl AST {
    pub fn forward(&mut self, ctx: &mut Context) {
        match self {
            AST::Expression { base, .. } => base.forward(ctx),
            AST::FunctionCall { name, ref arguments, ref options, .. } => {
                name.forward(ctx);
                match **name {
                    AST::Symbol(ref s)=>{
                        *self = evaluate_function(s, arguments, options, ctx)
                    }
                    _ => unimplemented!()
                }

            }
            AST::MultiplicativeExpression { expressions, .. } => {
                *self = evaluate_multiplicative(&expressions, ctx);
            }
            AST::List(v) => {
                for e in v {
                    e.forward(ctx)
                }
                // v.iter().map(AST::forward).collect();
            }
            AST::UnaryOperators { .. } => {}
            AST::InfixOperators { infix, lhs, rhs, .. } => {
                lhs.forward(ctx);
                rhs.forward(ctx);
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

fn evaluate_function(f: &Symbol, args:&Vec<AST>,kws: &BTreeMap<AST,AST>, _: &mut Context) -> AST {
    match f.name.as_str() {
        "wolfram_form" => {
            AST::string(&args[0].to_wolfram_string())
        },
        "tex_form" => {
            AST::string(&args[0].to_tex())
        },
        "first" => {
            internal::first(&args[0]).unwrap()
        },
        "last" => {
            internal::last(&args[0]).unwrap()
        },
        "length" => {
            internal::length(&args[0]).unwrap()
        },
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
