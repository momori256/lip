use crate::tokenizer::Token;

#[derive(Debug)]
pub enum ParserErr {
    Parse(String),
}

impl std::error::Error for ParserErr {}

impl std::fmt::Display for ParserErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    True,
    False,
}

pub fn parse(tokens: &[Token]) -> Result<Expr, ParserErr> {
    let (expr, _) = parse_internal(tokens)?;
    Ok(expr)
}

fn parse_internal(tokens: &[Token]) -> Result<(Expr, usize), ParserErr> {
    if tokens.is_empty() {
        return Err(ParserErr::Parse("no token".to_string()));
    }
    let first = tokens[0];
    if first != Token::Lparen {
        return match first {
            Token::True => Ok((Expr::True, 1)),
            Token::False => Ok((Expr::False, 1)),
            _ => Err(ParserErr::Parse(format!("invalid token `{first:?}`"))),
        };
    }
    if tokens.len() < 2 {
        return Err(ParserErr::Parse("`(` is not closed".to_string()));
    }
    let operator = tokens[1];
    match operator {
        Token::And | Token::Or => parse_binary_operator(operator, tokens),
        Token::Not => parse_unary_operator(operator, tokens),
        _ => Err(ParserErr::Parse(format!(
            "`{operator:?}` is not an operator"
        ))),
    }
}

/// Parse binary operator expression: (op expr expr).
fn parse_binary_operator(op: Token, tokens: &[Token]) -> Result<(Expr, usize), ParserErr> {
    let (lhs, ln) = parse_internal(&tokens[2..])?;
    let (rhs, rn) = parse_internal(&tokens[2 + ln..])?;
    let cnt = 2 + ln + rn;
    if tokens.len() <= cnt || tokens[cnt] != Token::Rparen {
        return Err(ParserErr::Parse("`(` is not closed".to_string()));
    }
    let lhs = Box::new(lhs);
    let rhs = Box::new(rhs);
    let cnt = cnt + 1;
    match op {
        Token::And => Ok((Expr::And(lhs, rhs), cnt)),
        Token::Or => Ok((Expr::Or(lhs, rhs), cnt)),
        _ => Err(ParserErr::Parse(format!(
            "`{op:?}` is not a binary operator"
        ))),
    }
}

/// Parse unary operator expression: (op expr).
fn parse_unary_operator(op: Token, tokens: &[Token]) -> Result<(Expr, usize), ParserErr> {
    let (expr, ln) = parse_internal(&tokens[2..])?;
    let cnt = 2 + ln;
    if tokens.len() <= cnt || tokens[cnt] != Token::Rparen {
        return Err(ParserErr::Parse("`(` is not closed".to_string()));
    }
    let expr = Box::new(expr);
    let cnt = cnt + 1;
    match op {
        Token::Not => Ok((Expr::Not(expr), cnt)),
        _ => Err(ParserErr::Parse(format!(
            "`{op:?}` is not an unary operator"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::{self, TokenizeErr};

    use super::{Expr::*, *};

    fn and(lhs: Expr, rhs: Expr) -> Expr {
        Expr::And(Box::new(lhs), Box::new(rhs))
    }

    fn or(lhs: Expr, rhs: Expr) -> Expr {
        Expr::Or(Box::new(lhs), Box::new(rhs))
    }

    fn not(expr: Expr) -> Expr {
        Expr::Not(Box::new(expr))
    }

    #[test]
    fn parse_and_succeed() -> Result<(), TokenizeErr> {
        let tokens = tokenizer::tokenize("(& true true)")?;
        let expr = parse_internal(&tokens);
        assert_eq!((and(True, True), tokens.len()), expr.unwrap());
        Ok(())
    }

    #[test]
    fn parse_or_succeed() -> Result<(), TokenizeErr> {
        let tokens = tokenizer::tokenize("(| true true)")?;
        let expr = parse_internal(&tokens);
        assert_eq!((or(True, True), tokens.len()), expr.unwrap());
        Ok(())
    }

    #[test]
    fn parse_nested_expr_succeed() -> Result<(), TokenizeErr> {
        let tokens = tokenizer::tokenize("(| (& true false) (^ (^ true)))")?;
        let expr = parse_internal(&tokens);
        assert_eq!(
            (or(and(True, False), not(not(True))), tokens.len()),
            expr.unwrap()
        );
        Ok(())
    }

    #[test]
    fn parse_unclosed_expr_fail() -> Result<(), TokenizeErr> {
        {
            let tokens = tokenizer::tokenize("(^ false")?;
            let expr = parse_internal(&tokens);
            assert!(expr.is_err());
        }
        {
            let tokens = tokenizer::tokenize("(& false true")?;
            let expr = parse_internal(&tokens);
            assert!(expr.is_err());
        }
        Ok(())
    }

    #[test]
    fn parse_invalid_expr_fail() -> Result<(), TokenizeErr> {
        let tokens = tokenizer::tokenize("(true)")?;
        let expr = parse_internal(&tokens);
        assert!(expr.is_err());
        Ok(())
    }
}
