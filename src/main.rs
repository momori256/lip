use std::io::{self, Write};

use lip::{environment::Environment, evaluator::eval, parser::parse, tokenizer::tokenize};

fn main() -> io::Result<()> {
    fn print(s: &str) -> io::Result<()> {
        print!("{s}");
        std::io::stdout().flush()
    }

    let mut env = Environment::default();
    loop {
        print("lip> ")?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();
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
