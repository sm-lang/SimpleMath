use crate::AST;

pub trait BoxArea {
    fn height(&self) -> usize {
        1
    }
    fn width(&self) -> usize {
        1
    }
}

impl BoxArea for AST {
    fn height(&self) -> usize {
        match self {
            AST::EmptyStatement => 0,
            AST::NewLine => 1,
            AST::Program(_) => 1,
            AST::Expression { .. } => 1,
            AST::FunctionCall { .. } => 1,
            AST::MultiplicativeExpression { .. } => 1,
            AST::AdditiveExpression { .. } => 1,
            AST::List(_) => 1,
            AST::UnaryOperators { .. } => 1,
            AST::InfixOperators { .. } => 1,
            _ => 1,
        }
    }
    fn width(&self) -> usize {
        match self {
            AST::EmptyStatement => 0,
            AST::NewLine => 1,
            AST::Program(_) => 1,
            AST::Expression { .. } => 1,
            AST::FunctionCall { .. } => 1,
            AST::MultiplicativeExpression { .. } => 1,
            AST::AdditiveExpression { .. } => 1,
            AST::List(v) => v.len(),
            AST::UnaryOperators { .. } => 1,
            AST::InfixOperators { .. } => 1,
            _ => 1,
        }
    }
}
