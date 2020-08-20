use crate::{parser::ParserSettings, Runner, SMResult, AST};

impl Default for Runner {
    fn default() -> Self {
        Self { parser: ParserSettings { file: String::from("anonymous"), refine: true }, ctx: Default::default() }
    }
}

impl Runner {
    pub(crate) fn parse(&mut self, s: &str) -> SMResult<()> {
        self.ctx.index += 1;
        self.ctx.inputs.insert(self.ctx.index, String::from(s));
        Ok(())
    }

    pub(crate) fn forward(&mut self) -> SMResult<()> {
        let input = self.ctx.inputs.get(&self.ctx.index).unwrap();
        let ast = self.parser.parse(input)?;
        let output = ast.forward(&mut self.ctx);
        self.ctx.outputs.insert(self.ctx.index, output);
        Ok(())
    }

    pub fn evaluate(&mut self, input: &str) -> SMResult<String> {
        self.parse(input)?;
        self.forward()?;
        let s = format!("{}", self.ctx.outputs.get(&self.ctx.index).unwrap());
        Ok(s)
    }

    pub fn last(&self) -> SMResult<AST> {
        let o = self.ctx.outputs.get(&self.ctx.index).unwrap();
        Ok(o.clone())
    }
}
