use sm_ast::{parser::ParserSettings, Runner, SMError, ToTex, AST};
use wasm_bindgen::prelude::*;

fn forward(input: &str) -> Runner {
    let parser = ParserSettings::default();

    let mut runner = match parser.parse(input) {
        Ok(o) => Runner::from(o),
        Err(e) => {}
    };

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
