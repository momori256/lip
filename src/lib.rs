pub mod environment;
pub mod evaluator;
pub mod parser;
pub mod repl;
pub mod tokenizer;

#[cfg(test)]
mod test_util {
    pub type TestResult = Result<(), Box<dyn std::error::Error>>;
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("hello, wasm.");
}
