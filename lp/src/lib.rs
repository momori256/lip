pub fn f() {
    println!("test");
}

type LpErr = Box<dyn std::error::Error>;

mod tokenizer {
    use super::LpErr;

    #[derive(Debug, PartialEq, Eq)]
    pub enum Operator {
        And,
        Or,
        Not,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum Token {
        Lparen,
        Rparen,
        Bool(bool),
        Operator(Operator),
    }

    pub fn tokenize(expr: &str) -> Result<Vec<Token>, LpErr> {
        let tokens: Result<Vec<Token>, _> = expr
            .replace("(", "( ")
            .replace(")", " )")
            .split_ascii_whitespace()
            .map(|s| match s {
                "(" => Ok(Token::Lparen),
                ")" => Ok(Token::Rparen),
                "T" => Ok(Token::Bool(true)),
                "F" => Ok(Token::Bool(false)),
                "&" => Ok(Token::Operator(Operator::And)),
                "|" => Ok(Token::Operator(Operator::Or)),
                "^" => Ok(Token::Operator(Operator::Not)),
                _ => Err(format!("invalid token `{s}`")),
            })
            .collect();
        Ok(tokens?)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn tokenize_works() -> Result<(), LpErr> {
            let tokens = tokenize("( ) T F & | ^")?;
            assert_eq!(
                vec![
                    Token::Lparen,
                    Token::Rparen,
                    Token::Bool(true),
                    Token::Bool(false),
                    Token::Operator(Operator::And),
                    Token::Operator(Operator::Or),
                    Token::Operator(Operator::Not)
                ],
                tokens
            );
            Ok(())
        }
    }
}

pub fn parse() {
    todo!()
}

pub fn eval() {
    todo!()
}
