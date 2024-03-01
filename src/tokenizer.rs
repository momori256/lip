#[derive(Debug)]
pub enum TokenizeErr {
    Parse(String),
}

impl std::error::Error for TokenizeErr {}

impl std::fmt::Display for TokenizeErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to tokenize")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Lparen,
    Rparen,
    And,
    Or,
    Not,
    True,
    False,
    If,
    Def,
    Ident(String),
}

impl Token {
    fn parse(str: &str) -> Result<Self, TokenizeErr> {
        use Token::*;
        match str {
            "(" => Ok(Lparen),
            ")" => Ok(Rparen),
            "&" => Ok(And),
            "|" => Ok(Or),
            "^" => Ok(Not),
            "T" => Ok(True),
            "F" => Ok(False),
            "if" => Ok(If),
            "def" => Ok(Def),
            str if str.chars().all(|c| c.is_ascii_lowercase()) => Ok(Ident(str.to_string())),
            _ => Err(TokenizeErr::Parse(format!("Invalid token `{str}`"))),
        }
    }
}

pub fn tokenize(expr: &str) -> Result<Vec<Token>, TokenizeErr> {
    expr.replace('(', "( ")
        .replace(')', " )")
        .split_whitespace()
        .map(Token::parse)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::Token::*;
    use super::*;

    #[test]
    fn tokenize_valid_tokens_parsed_successfully() {
        let tokens = tokenize("( ) & | ^ T F if def");
        assert_eq!(
            vec![Lparen, Rparen, And, Or, Not, True, False, If, Def],
            tokens.unwrap()
        );
    }

    #[test]
    fn tokenize_identifier_succeed() {
        let tokens = tokenize("myvar abc");
        assert_eq!(
            vec![Ident("myvar".to_string()), Ident("abc".to_string())],
            tokens.unwrap()
        );
    }

    #[test]
    fn tokenize_invalid_token_cannot_be_parsed() {
        let tokens = tokenize("( ) & | ^ T F $");
        match tokens {
            Err(TokenizeErr::Parse(e)) => assert_eq!("Invalid token `$`", e),
            _ => panic!(),
        };
    }
}
