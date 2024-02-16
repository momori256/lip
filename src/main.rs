use std::io::{self, Write};

use lip::{evaluator::eval, parser::parse, tokenizer::tokenize};

fn main() -> io::Result<()> {
    fn print(s: &str) -> io::Result<()> {
        print!("{s}");
        std::io::stdout().flush()
    }

    loop {
        print("lip> ")?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if input == ":exit" {
            break;
        }
        let tokens = tokenize(&input);
        if let Err(e) = tokens {
            print(&format!("Failed to tokenize: {e:?}\n"))?;
            continue;
        }
        let expr = parse(&tokens.unwrap());
        if let Err(e) = expr {
            print(&format!("Failed to parse: {e:?}\n"))?;
            continue;
        }
        let output = eval(&expr.unwrap());
        print(&format!("=> {output}\n"))?;
    }
    Ok(())
}
