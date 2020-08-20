use crate::{ast::{Parameter, Symbol}, internal, Context, AST, SMError, SMResult};
use num::{traits::Pow, ToPrimitive};

impl AST {
    pub fn forward(&self, ctx: &mut Context) -> SMResult<AST> {
        let out = match self {
            AST::EmptyStatement | AST::NewLine | AST::Boolean(..) | AST::Integer(..) | AST::Decimal(..) | AST::Symbol(..) | AST::String(..) => {
                self.clone()
            }
            AST::Program(_) => unimplemented!(),
            AST::Expression { base, .. } => base.forward(ctx),
            AST::Function(s, p) => match p.len() {
                0 => AST::Symbol(s.clone()),
                1 => evaluate_function(s, p[0].clone(), ctx),
                _ => {
                    return Err(SMError::Unimplemented(format!("Unimplemented Function: forward at line {}", line!())));
                }
            },
        };
        Ok(out)
    }
}

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
fn evaluate_function(f: &Symbol, p: Parameter, ctx: &mut Context) -> SMResult<AST> {
    let args = p.arguments;
    let out = match f.name.as_str() {
        "first" => internal::first(&args[0]).unwrap(),
        "last" => internal::last(&args[0]).unwrap(),
        "length" => internal::length(&args[0]).unwrap(),
        "factorial" => internal::factorial(&args[0]).unwrap(),
        "fibonacci" => internal::fibonacci(&args[0]).unwrap(),
        "plus" => evaluate_additive(&args, ctx),
        "times" => evaluate_multiplicative(&args, ctx),
        "power" => evaluate_power(&args, ctx),
        _ => {
            return Err(SMError::Unimplemented(format!("Unimplemented Function: evaluate_function at line {}", line!())));
        }
    };
    Ok(out)
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
