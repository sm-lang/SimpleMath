use wasm_bindgen::prelude::*;
use sm_ast::parser::ParserSettings;
use sm_ast::{Runner, ToTex};


fn forward(input: &str) -> Runner {
    let parser = ParserSettings::default();
    let mut runner = Runner::from(parser.parse(input));
    runner.forward();
    return runner;
}

#[wasm_bindgen]
pub fn result(input: &str) -> String {
    format!("{}", forward(input).ast)
}

#[wasm_bindgen]
pub fn result_tex(input: &str) -> String {
    format!("{}", forward(input).ast.to_tex())
}