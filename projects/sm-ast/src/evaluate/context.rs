use crate::evaluate::{Context};
use crate::AST;
use num::traits::Pow;
use num::ToPrimitive;

impl AST {
    pub fn forward(&mut self, ctx: &mut Context) {
        match self {
            AST::EmptyStatement => {}
            AST::NewLine => {}
            AST::Program(_) => {}
            AST::Expression { base, .. } => {
                base.forward(ctx)
            }
            AST::FunctionCall { .. } => {}
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
                    "*"|"×" => {
                        *self = evaluate_multiplicative(&vec![*lhs.clone(), *rhs.clone()], ctx);
                    }
                    "^" => {
                        *self = evaluate_power(&vec![*lhs.clone(), *rhs.clone()], ctx);
                    }
                    _ => ()
                }
            }
            _ => (),
        }
    }
}

fn evaluate_additive(vec: &[AST], _: &mut Context) -> AST {
    match vec {
        [AST::Integer(lhs), AST::Integer(rhs)] => {
            AST::Integer(lhs + rhs)
        }
        _ => {
            println!("{:?}",vec);
            unimplemented!()
        }
    }
}

fn evaluate_multiplicative(vec: &[AST], _: &mut Context) -> AST {
    match vec {
        [AST::Integer(lhs), AST::Integer(rhs)] => {
            AST::Integer(lhs * rhs)
        }
        _ => {
            println!("{:?}",vec);
            unimplemented!()
        }
    }
}

fn evaluate_power(vec: &[AST], _: &mut Context) -> AST {
    match vec {
        [AST::Integer(lhs), AST::Integer(rhs)] => {
            match rhs.to_u64() {
                None => AST::Integer(lhs.clone()),
                Some(s) => AST::Integer(lhs.pow(s)),
            }
        }
        _ => {
            println!("{:?}",vec);
            unimplemented!()
        }
    }
}