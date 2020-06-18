use crate::{AST, Runner, Context};

impl From<AST> for Runner {
    fn from(ast: AST) -> Self {
        let ctx = Context::default();
        Self { ast, ctx }
    }
}

impl Runner {
    pub fn evaluate(&mut self){
        let refined  = self.ast.rewrite();
        self.ast = refined
    }
}