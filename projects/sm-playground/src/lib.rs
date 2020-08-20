use lazy_static::lazy_static;
use sm_ast::{Runner, ToTex};
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

lazy_static! {
    static ref RUNNER: Mutex<Runner> = Mutex::new(Runner::default());
}

#[wasm_bindgen]
pub fn result(input: &str) -> String {
    let r = &mut RUNNER.lock().unwrap();
    r.evaluate(input).unwrap()
}

#[wasm_bindgen]
pub fn result_tex() -> String {
    let r = &mut RUNNER.lock().unwrap();
    r.last().unwrap().to_tex()
}
