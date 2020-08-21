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
    match r.evaluate(input) {
        Ok(o) => o,
        Err(e) => format!("{:?}", e),
    }
}

#[wasm_bindgen]
pub fn result_tex() -> String {
    let r = &mut RUNNER.lock().unwrap();
    match r.last() {
        Ok(o) => o.to_tex(),
        Err(e) => format!("{:?}", e),
    }
}
