use sm_ast::{parser::ParserSettings, Runner, SMError, ToTex, AST};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn new_runner() -> Runner {
    Runner::default()
}

#[wasm_bindgen]
pub fn result(r: &mut Runner, input: &str) -> String {
    r.evaluate(input).unwrap()
}

#[wasm_bindgen]
pub fn result_tex(r: &Runner) -> String {
    r.last().unwrap().to_tex()
}
