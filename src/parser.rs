use crate::tokenizer::Token;

#[derive(Debug)]
pub enum ParserErr {
    Parse(String),
}

impl std::error::Error for ParserErr {}

impl std::fmt::Display for ParserErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse")
    }
}

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

#[derive(Debug, PartialEq, Eq)]
pub struct If {
    pub cond: Box<Expr>,
    pub then: Box<Expr>,
    pub other: Box<Expr>,
}

impl If {
    pub fn new(cond: Expr, then: Expr, other: Expr) -> Self {
        Self {
            cond: Box::new(cond),
            then: Box::new(then),
            other: Box::new(other),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Bool(bool),
    Operator(Operator),
    Call(Operator, Vec<Expr>),
    If(If),
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
            Token::True => Ok((Expr::Bool(true), 1)),
            Token::False => Ok((Expr::Bool(false), 1)),
            Token::And => Ok((Expr::Operator(Operator::And), 1)),
            Token::Or => Ok((Expr::Operator(Operator::Or), 1)),
            Token::Not => Ok((Expr::Operator(Operator::Not), 1)),
            _ => Err(ParserErr::Parse(format!("invalid token `{first:?}`"))),
        };
    }
    if tokens[1] == Token::If {
        return parse_if(tokens);
    }
    parse_call(tokens)
}

fn parse_call(tokens: &[Token]) -> Result<(Expr, usize), ParserErr> {
    let len = tokens.len();
    if len < 3 {
        return Err(ParserErr::Parse("call is too short".to_string()));
    }

    if tokens[0] != Token::Lparen {
        return Err(ParserErr::Parse(format!(
            "call must start with `(`, not `{:?}`",
            tokens[0]
        )));
    }
    let operator = match tokens[1] {
        Token::And => Operator::And,
        Token::Or => Operator::Or,
        Token::Not => Operator::Not,
        _ => {
            return Err(ParserErr::Parse(format!(
                "`{:?}` is not an operator",
                tokens[1]
            )));
        }
    };
    let mut operands = Vec::new();
    let mut p = 2;
    while p < len && tokens[p] != Token::Rparen {
        let (expr, cnt) = parse_internal(&tokens[p..])?;
        operands.push(expr);
        p += cnt;
    }
    if p >= len || tokens[p] != Token::Rparen {
        return Err(ParserErr::Parse("call is not closed with `)`".to_string()));
    }
    Ok((Expr::Call(operator, operands), p + 1))
}

fn parse_if(tokens: &[Token]) -> Result<(Expr, usize), ParserErr> {
    let len = tokens.len();
    if len < 6 {
        return Err(ParserErr::Parse("if expression is too short".to_string()));
    }
    if tokens[0] != Token::Lparen || tokens[1] != Token::If {
        return Err(ParserErr::Parse(format!(
            "if expression must start with `( if`, not `{:?} {:?}`",
            tokens[0], tokens[1]
        )));
    }
    let mut p = 2;
    let (cond, cnt) = parse_internal(&tokens[p..])?;
    p += cnt;
    let (then, cnt) = parse_internal(&tokens[p..])?;
    p += cnt;
    let (other, cnt) = parse_internal(&tokens[p..])?;
    p += cnt;
    if tokens[p] != Token::Rparen {
        return Err(ParserErr::Parse(
            "if expression is not closed with `)`".to_string(),
        ));
    }
    Ok((Expr::If(If::new(cond, then, other)), p + 1))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer;

    fn and(exprs: Vec<Expr>) -> Expr {
        Expr::Call(Operator::And, exprs)
    }

    fn or(exprs: Vec<Expr>) -> Expr {
        Expr::Call(Operator::Or, exprs)
    }

    fn not(exprs: Vec<Expr>) -> Expr {
        Expr::Call(Operator::Not, exprs)
    }

    fn if_expr(cond: Expr, then: Expr, other: Expr) -> Expr {
        Expr::If(If::new(cond, then, other))
    }

    #[test]
    fn parse_bool_succeed() -> Result<(), Box<dyn std::error::Error>> {
        let tokens = tokenizer::tokenize("T")?;
        let (expr, cnt) = parse_internal(&tokens)?;
        assert_eq!(tokens.len(), cnt);
        assert_eq!(Expr::Bool(true), expr);
        Ok(())
    }

    #[test]
    fn parse_operator_succeed() -> Result<(), Box<dyn std::error::Error>> {
        for (str, operator) in [
            ("&", Operator::And),
            ("|", Operator::Or),
            ("^", Operator::Not),
        ] {
            let tokens = tokenizer::tokenize(str)?;
            let (expr, cnt) = parse_internal(&tokens)?;
            assert_eq!(tokens.len(), cnt);
            assert_eq!(Expr::Operator(operator), expr);
        }
        Ok(())
    }

    #[test]
    fn parse_call_and_succeed() -> Result<(), Box<dyn std::error::Error>> {
        let tokens = tokenizer::tokenize("(& T (| F F T) (^ T))")?;
        let (expr, cnt) = parse_internal(&tokens)?;
        assert_eq!(tokens.len(), cnt);
        assert_eq!(
            and(vec![
                Expr::Bool(true),
                or(vec![Expr::Bool(false), Expr::Bool(false), Expr::Bool(true)]),
                not(vec![Expr::Bool(true)])
            ]),
            expr
        );
        Ok(())
    }

    #[test]
    fn parse_if_succeed() -> Result<(), Box<dyn std::error::Error>> {
        let tokens = tokenizer::tokenize("(if (& T T) T (| F F))")?;
        let (expr, cnt) = parse_internal(&tokens)?;
        assert_eq!(tokens.len(), cnt);
        assert_eq!(
            if_expr(
                and(vec![Expr::Bool(true), Expr::Bool(true)]),
                Expr::Bool(true),
                or(vec![Expr::Bool(false), Expr::Bool(false)])
            ),
            expr
        );
        Ok(())
    }

    #[test]
    fn parse_invalid_expr_fail() -> Result<(), Box<dyn std::error::Error>> {
        let tokens = tokenizer::tokenize("(& T F")?;
        match parse_internal(&tokens) {
            Err(ParserErr::Parse(_)) => (),
            _ => panic!(),
        }
        Ok(())
    }
}
