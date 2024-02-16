use crate::parser::Expr;

pub fn eval(expr: &Expr) -> bool {
    match expr {
        Expr::And(lhs, rhs) => {
            let lhs = eval(lhs);
            let rhs = eval(rhs);
            lhs && rhs
        }
        Expr::Or(lhs, rhs) => {
            let lhs = eval(lhs);
            let rhs = eval(rhs);
            lhs || rhs
        }
        Expr::Not(expr) => {
            let expr = eval(expr);
            !expr
        },
        Expr::True => true,
        Expr::False => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser, tokenizer};

    use super::*;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn eval_simple_expr_works() -> TestResult {
        // true & false
        let tokens = tokenizer::tokenize("(& true false)")?;
        let expr = parser::parse(&tokens)?;
        let output = eval(&expr);
        assert_eq!(false, output);
        Ok(())
    }

    #[test]
    fn eval_works() -> TestResult {
        // !(true & (false | false))
        let tokens = tokenizer::tokenize("(^ (& true (| false false)))")?;
        let expr = parser::parse(&tokens)?;
        let output = eval(&expr);
        assert_eq!(true, output);
        Ok(())
    }
}
