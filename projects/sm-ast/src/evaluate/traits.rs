use crate::{parser::ParserSettings, Runner, SMResult};

impl Default for Runner {
    fn default() -> Self {
        Self { parser: ParserSettings { file: String::from("anonymous"), refine: true }, ctx: Default::default() }
    }
}

impl Runner {
    pub fn evaluate(&mut self, input: &str) -> SMResult<()> {
        let parsed = self.parser.parse(input)?;
        let refined = parsed.rewrite();
        Ok(())
    }
}
