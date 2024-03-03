#[derive(Debug)]
pub enum LpErr {
    Tokenize(String),
    Parse(String),
    Eval(String),
}

pub mod tokenizer {
    use super::LpErr;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Operator {
        And,
        Or,
        Not,
    }

    impl std::fmt::Display for Operator {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Operator::And => write!(f, "&"),
                Operator::Or => write!(f, "|"),
                Operator::Not => write!(f, "^"),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Token {
        Lparen,
        Rparen,
        Bool(bool),
        Operator(Operator),
        If,
    }

    impl std::fmt::Display for Token {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Token::Lparen => write!(f, "("),
                Token::Rparen => write!(f, ")"),
                Token::Bool(b) => write!(f, "{b}"),
                Token::Operator(o) => write!(f, "{o}"),
                Token::If => write!(f, "if"),
            }
        }
    }

    pub fn tokenize(expr: &str) -> Result<Vec<Token>, LpErr> {
        expr.replace('(', "( ")
            .replace(')', " )")
            .split_ascii_whitespace()
            .map(|s| match s {
                "(" => Ok(Token::Lparen),
                ")" => Ok(Token::Rparen),
                "T" => Ok(Token::Bool(true)),
                "F" => Ok(Token::Bool(false)),
                "&" => Ok(Token::Operator(Operator::And)),
                "|" => Ok(Token::Operator(Operator::Or)),
                "^" => Ok(Token::Operator(Operator::Not)),
                "if" => Ok(Token::If),
                _ => Err(LpErr::Tokenize(format!("invalid token `{s}`"))),
            })
            .collect::<Result<Vec<Token>, LpErr>>()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn tokenize_works() -> Result<(), LpErr> {
            let tokens = tokenize("( ) T F & | ^ if")?;
            assert_eq!(
                vec![
                    Token::Lparen,
                    Token::Rparen,
                    Token::Bool(true),
                    Token::Bool(false),
                    Token::Operator(Operator::And),
                    Token::Operator(Operator::Or),
                    Token::Operator(Operator::Not),
                    Token::If,
                ],
                tokens
            );
            Ok(())
        }
    }
}

pub mod parser {
    use super::{
        tokenizer::{Operator, Token},
        LpErr,
    };

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum Expr {
        Bool(bool),
        Operator(Operator),
        Call(Box<Expr>, Vec<Expr>),
        If(Box<Expr>, Box<Expr>, Box<Expr>),
    }

    pub fn parse(tokens: &[Token]) -> Result<Expr, LpErr> {
        let (expr, _) = parse_internal(tokens)?;
        Ok(expr)
    }

    fn parse_internal(tokens: &[Token]) -> Result<(Expr, usize), LpErr> {
        if tokens[0] != Token::Lparen {
            return match tokens[0] {
                Token::Bool(b) => Ok((Expr::Bool(b), 1)),
                Token::Operator(o) => Ok((Expr::Operator(o), 1)),
                _ => Err(LpErr::Parse(format!("invalid expression: `{}`", tokens[0]))),
            };
        }
        if tokens[1] == Token::If {
            return parse_if(tokens);
        }

        let mut p = 1;
        let (operator, consumed) = parse_internal(&tokens[p..])?;
        p += consumed;
        let mut operands = vec![];
        while tokens[p] != Token::Rparen {
            let (expr, consumed) = parse_internal(&tokens[p..])?;
            operands.push(expr);
            p += consumed;
        }
        Ok((Expr::Call(Box::new(operator), operands), p + 1))
    }

    fn parse_if(tokens: &[Token]) -> Result<(Expr, usize), LpErr> {
        let mut p = 2;
        let (cond, consumed) = parse_internal(&tokens[p..])?;
        p += consumed;
        let (then, consumed) = parse_internal(&tokens[p..])?;
        p += consumed;
        let (r#else, consumed) = parse_internal(&tokens[p..])?;
        p += consumed;
        Ok((
            Expr::If(Box::new(cond), Box::new(then), Box::new(r#else)),
            p + 1,
        ))
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::tokenizer;

        const T: Expr = Expr::Bool(true);
        const F: Expr = Expr::Bool(false);

        fn and(operands: &[Expr]) -> Expr {
            Expr::Call(Box::new(Expr::Operator(Operator::And)), operands.to_vec())
        }

        fn or(operands: &[Expr]) -> Expr {
            Expr::Call(Box::new(Expr::Operator(Operator::Or)), operands.to_vec())
        }

        fn not(operands: &[Expr]) -> Expr {
            Expr::Call(Box::new(Expr::Operator(Operator::Not)), operands.to_vec())
        }

        fn r#if(cond: Expr, then: Expr, r#else: Expr) -> Expr {
            Expr::If(Box::new(cond), Box::new(then), Box::new(r#else))
        }

        #[test]
        fn parse_works() -> Result<(), LpErr> {
            let tokens = tokenizer::tokenize("(^ (& T F (| F F)))")?;
            let (expr, consumed) = parse_internal(&tokens)?;
            assert_eq!(tokens.len(), consumed);
            assert_eq!(not(&[and(&[T, F, or(&[F, F])])]), expr);
            Ok(())
        }

        #[test]
        fn parse_if_works() -> Result<(), LpErr> {
            let tokens = tokenizer::tokenize("(if T & |)")?;
            let (expr, consumed) = parse_internal(&tokens)?;
            assert_eq!(tokens.len(), consumed);
            assert_eq!(
                r#if(
                    T,
                    Expr::Operator(Operator::And),
                    Expr::Operator(Operator::Or)
                ),
                expr
            );
            Ok(())
        }
    }
}

pub mod evaluator {
    use super::{parser::Expr, tokenizer::Operator, LpErr};

    #[derive(Debug, PartialEq, Eq)]
    pub enum Value {
        Bool(bool),
        Operator(Operator),
    }

    impl std::fmt::Display for Value {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Value::Bool(b) => write!(f, "{b}"),
                Value::Operator(o) => write!(f, "primitive operator: {o}"),
            }
        }
    }

    pub fn eval(expr: &Expr) -> Result<Value, LpErr> {
        match expr {
            Expr::Bool(b) => Ok(Value::Bool(*b)),
            Expr::Operator(o) => Ok(Value::Operator(*o)),
            Expr::Call(operator, operands) => {
                let operands: Vec<bool> = operands
                    .iter()
                    .map(|o| match eval(o) {
                        Ok(Value::Bool(b)) => Ok(b),
                        _ => Err(LpErr::Eval(format!("invalid operand: {o:?}"))),
                    })
                    .collect::<Result<Vec<bool>, LpErr>>()?;

                let value = match eval(operator)? {
                    Value::Operator(o) => match o {
                        Operator::And => operands.into_iter().all(|o| o),
                        Operator::Or => operands.into_iter().any(|o| o),
                        Operator::Not => {
                            let len = operands.len();
                            if len != 1 {
                                return Err(LpErr::Eval(format!(
                                    "not must have 1 operands, not {len}"
                                )));
                            }
                            !operands[0]
                        }
                    },
                    value => return Err(LpErr::Eval(format!("invalid operator: {value}"))),
                };
                Ok(Value::Bool(value))
            }
            Expr::If(cond, then, r#else) => {
                let cond = match eval(cond)? {
                    Value::Bool(cond) => cond,
                    value => return Err(LpErr::Eval(format!("invalid condition: {value}"))),
                };
                eval(if cond { then } else { r#else })
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::{parser, tokenizer};

        #[test]
        fn eval_works() -> Result<(), LpErr> {
            // !(true & false & (false | false)) => true
            let tokens = tokenizer::tokenize("(^ (& T F (| F F)))")?;
            let expr = parser::parse(&tokens)?;
            let value = eval(&expr)?;
            assert_eq!(Value::Bool(true), value);
            Ok(())
        }

        #[test]
        fn eval_if_works() -> Result<(), LpErr> {
            // (if T & |) => &, (& T F) => F
            let tokens = tokenizer::tokenize("((if T & |) T F)")?;
            let expr = parser::parse(&tokens)?;
            let value = eval(&expr)?;
            assert_eq!(Value::Bool(false), value);
            Ok(())
        }
    }
}
