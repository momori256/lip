use std::io::{self, BufRead, Write};

use crate::{environment::Environment, evaluator::eval, parser::parse, tokenizer::tokenize};

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
