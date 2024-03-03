use std::io::{self, Write};

use lp::{evaluator::eval, parser::parse, tokenizer::tokenize};

fn main() -> io::Result<()> {
    loop {
        print!("lp> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim() == ":exit" {
            break;
        }

        let result = tokenize(&input)
            .and_then(|tokens| parse(&tokens))
            .and_then(|expr| eval(&expr));

        match result {
            Ok(value) => {
                println!("=> {value}");
                io::stdout().flush()?;
            }
            Err(e) => eprintln!("error: {e:?}"),
        }
    }
    Ok(())
}
