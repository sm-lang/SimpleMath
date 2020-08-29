use crate::AST;

// todo: remove redundant forward
impl AST {
    pub fn rewrite(&self) -> AST {
        match self {
            AST::Function { .. } => self.clone(),
            // ```rs
            // AST::AdditiveExpression { terms, position } => {
            // let mut new = vec![];
            // for e in terms {
            // match e {
            // AST::Integer(i) => {
            // if i.is_zero() {
            // continue;
            // }
            // else {
            // new.push(e.clone())
            // }
            // }
            // AST::AdditiveExpression { .. } => new.push(e.clone()),
            // _ => new.push(e.clone()),
            // }
            // }
            // AST::AdditiveExpression { terms: new, position: position.clone() }
            // }
            //
            // AST::MultiplicativeExpression { .. } => self.clone(),
            // ```
            // AST::List(..) => self.clone(),
            _ => self.clone(),
        }
    }
}
