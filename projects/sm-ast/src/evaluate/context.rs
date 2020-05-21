use crate::evaluate::{Context};
use crate::AST;

impl AST {
    pub fn forward(&mut self,ctx:&mut Context) {
        match self {
            AST::EmptyStatement => {},
            AST::NewLine => {},
            AST::Program(_) => {},
            AST::Expression { base, eos, .. } => {
                if *eos {
                    *self = AST::EmptyStatement
                }
            },
            AST::FunctionCall { .. } => {},
            AST::MultiplicativeExpression { .. } => {},
            AST::List(v) => {
                for e in v {
                    e.forward(ctx)
                }
                // v.iter().map(AST::forward).collect();
            },
            AST::UnaryOperators { .. } => {},
            AST::InfixOperators { .. } => {},
            _ => (),
        }
    }
}
