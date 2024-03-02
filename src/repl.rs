use std::io::{self, BufRead, Write};
use wasm_bindgen::prelude::*;

use crate::{
    environment::Environment,
    evaluator::{self, eval, Value},
    parser::{self, parse},
    tokenizer::{self, tokenize},
};

#[wasm_bindgen]
pub struct Repl {
    env: Environment,
}

#[wasm_bindgen]
impl Repl {
    pub fn new() -> Self {
        Self {
            env: Environment::default(),
        }
    }

    pub fn eval(&mut self, expr: &str) -> String {
        format!("{:?}", self.eval_internal(expr))
    }

    fn eval_internal(&mut self, expr: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let tokens = tokenizer::tokenize(expr)?;
        let expr = parser::parse(&tokens)?;
        Ok(evaluator::eval(&expr, &mut self.env)?)
    }
}

impl std::default::Default for Repl {
    fn default() -> Self {
        Self::new()
    }
}

pub fn run<R: BufRead, W: Write>(input: &mut R, output: &mut W) -> io::Result<()> {
    let mut print = move |s: &str| {
        write!(output, "{s}")?;
        output.flush()
    };

    let mut env = Environment::default();
    loop {
        print("lip> ")?;

        let mut buf = String::new();
        input.read_line(&mut buf)?;
        let input = buf.trim();
        if input == ":exit" {
            break;
        }
        if input == ":env" {
            print(&format!("{env:?}\n"))?;
            continue;
        }
        let tokens = tokenize(input);
        if let Err(e) = tokens {
            print(&format!("Failed to tokenize: {e:?}\n"))?;
            continue;
        }
        let expr = parse(&tokens.unwrap());
        if let Err(e) = expr {
            print(&format!("Failed to parse: {e:?}\n"))?;
            continue;
        }
        let value = eval(&expr.unwrap(), &mut env);
        if let Err(e) = value {
            print(&format!("Failed to evalueate: {e:?}\n"))?;
            continue;
        }
        print(&format!("{}\n", value.unwrap()))?;
    }
    Ok(())
}
