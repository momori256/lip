pub fn f() {
    println!("test");
}

#[derive(Debug)]
pub enum LpErr {
    Tokenize(String),
    Parse(String),
}

mod tokenizer {
    use super::LpErr;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Operator {
        And,
        Or,
        Not,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Token {
        Lparen,
        Rparen,
        Bool(bool),
        Operator(Operator),
    }

    pub fn tokenize(expr: &str) -> Result<Vec<Token>, LpErr> {
        expr.replace("(", "( ")
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
                _ => Err(LpErr::Tokenize(format!("invalid token `{s}`"))),
            })
            .collect::<Result<Vec<Token>, LpErr>>()
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

mod Parser {
    use super::{
        tokenizer::{self, Operator, Token},
        LpErr,
    };

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum Expr {
        Bool(bool),
        Call(Operator, Vec<Expr>),
    }

    pub fn parse(tokens: &[Token]) -> Result<Expr, LpErr> {
        let (expr, _) = parse_internal(tokens)?;
        Ok(expr)
    }

    fn parse_internal(tokens: &[Token]) -> Result<(Expr, usize), LpErr> {
        if tokens[0] != Token::Lparen {
            return match tokens[0] {
                Token::Bool(b) => Ok((Expr::Bool(b), 1)),
                _ => Err(LpErr::Parse(format!("invalid expression: `{tokens:?}`"))),
            };
        }
        let operator = match tokens[1] {
            Token::Operator(o) => o,
            _ => return Err(LpErr::Parse(format!("invalid operator: `{:?}`", tokens[2]))),
        };
        let mut p = 2;
        let mut operands = vec![];
        while tokens[p] != Token::Rparen {
            let (expr, consumed) = parse_internal(&tokens[p..])?;
            operands.push(expr);
            p += consumed;
        }
        Ok((Expr::Call(operator, operands), p + 1))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const T: Expr = Expr::Bool(true);
        const F: Expr = Expr::Bool(false);

        fn and(operands: &[Expr]) -> Expr {
            Expr::Call(Operator::And, operands.to_vec())
        }

        fn or(operands: &[Expr]) -> Expr {
            Expr::Call(Operator::Or, operands.to_vec())
        }

        fn not(operands: &[Expr]) -> Expr {
            Expr::Call(Operator::Not, operands.to_vec())
        }

        #[test]
        fn parse_works() -> Result<(), LpErr> {
            let tokens = tokenizer::tokenize("(^ (& T F (| F F)))")?;
            let (expr, consumed) = parse_internal(&tokens)?;
            assert_eq!(tokens.len(), consumed);
            assert_eq!(not(&[and(&[T, F, or(&[F, F])])]), expr);
            Ok(())
        }
    }
}

pub fn eval() {
    todo!()
}
