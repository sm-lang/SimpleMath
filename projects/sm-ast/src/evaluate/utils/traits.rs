use crate::{
    evaluate::{Context, Runner},
    AST,
};

impl Default for Context {
    fn default() -> Self {
        Context {}
    }
}

impl Default for Runner {
    fn default() -> Self {
        Self { ast: AST::EmptyStatement, ctx: Context::default() }
    }
}

impl From<AST> for Runner {
    fn from(e: AST) -> Self {
        Runner { ast: e, ctx: Default::default() }
    }
}

impl Runner {
    pub fn forward(&mut self) {
        // println!("ok");
        self.ast.forward(&mut self.ctx)
    }
}
