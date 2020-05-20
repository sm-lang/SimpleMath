use crate::evaluate::Context;
use crate::AST;
use crate::parser::ParserSettings;

impl Context {
    pub fn parse(&mut self, s:&str) {
        let parser = ParserSettings{
            file: String::from("anonymous"),
            refine: true
        };
        self.ast = parser.parse(s)
    }

    pub fn forward(&mut self){

    }
}


