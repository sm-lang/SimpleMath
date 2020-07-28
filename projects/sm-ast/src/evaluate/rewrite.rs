use crate::AST;

// todo: remove redundant forward
impl AST {
    pub fn rewrite(&self) -> AST {
        match self {
            AST::Expression { base, .. } => base.rewrite(),
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
            //AST::List(..) => self.clone(),
            AST::UnaryOperators { .. } => self.clone(),
            AST::InfixOperators { .. } => {
                self.clone()
                // ,  `Vec<&mut Box<AST>>` ->  `&mut [AST]`
                /*
                match infix.as_str() {
                    "+" => {
                        let terms = vec![(**lhs).clone(), (**rhs).clone()];
                        let ast = AST::AdditiveExpression { terms, position: (*position).clone() };
                        ast.rewrite()
                    }
                    "*" | "×" => {
                        let terms = vec![(**lhs).clone(), (**rhs).clone()];
                        let ast = AST::MultiplicativeExpression { terms, position: (*position).clone() };
                        ast.rewrite()
                    }
                    _ => return self.clone(),
                }

                 */
            }
            _ => self.clone(),
        }
    }
}
