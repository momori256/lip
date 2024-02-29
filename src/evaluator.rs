use crate::parser::{self, Expr};

#[derive(Debug)]
pub enum EvalErr {
    Eval(String),
}

impl std::error::Error for EvalErr {}

impl std::fmt::Display for EvalErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to evaluate expression")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    Bool(bool),
    Operand(parser::Operator),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{b}"),
            Value::Operand(o) => write!(f, "primitive operator: {o}"),
        }
    }
}

fn eval_bool_args(args: &[Expr]) -> Result<impl Iterator<Item = bool>, EvalErr> {
    let args: Vec<Value> = args
        .into_iter()
        .map(|arg| eval(arg))
        .collect::<Result<Vec<Value>, EvalErr>>()?;
    if args.iter().any(|arg| !matches!(arg, Value::Bool(_))) {
        return Err(EvalErr::Eval("`operand must be bool".to_string()));
    }
    let args = args.into_iter().map(|arg| match arg {
        Value::Bool(b) => b,
        _ => unreachable!(),
    });
    Ok(args)
}

pub fn eval(expr: &Expr) -> Result<Value, EvalErr> {
    match expr {
        Expr::Bool(b) => Ok(Value::Bool(*b)),
        Expr::Operator(o) => Ok(Value::Operand(*o)),
        Expr::Call(operator, args) => match operator {
            parser::Operator::And => {
                let result = eval_bool_args(args)?.fold(true, |acc, b| acc & b);
                Ok(Value::Bool(result))
            }
            parser::Operator::Or => {
                let result = eval_bool_args(args)?.fold(false, |acc, b| acc | b);
                Ok(Value::Bool(result))
            }
            parser::Operator::Not => {
                let args: Vec<bool> = eval_bool_args(args)?.collect();
                if args.len() != 1 {
                    return Err(EvalErr::Eval(format!(
                        "the number of arguments of {operator} must be 1"
                    )));
                }
                Ok(Value::Bool(!args[0]))
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parser, tokenizer};

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn eval_bool_succeed() -> TestResult {
        let tokens = tokenizer::tokenize("T")?;
        let expr = parser::parse(&tokens)?;
        let value = eval(&expr)?;
        assert_eq!(Value::Bool(true), value);
        Ok(())
    }

    #[test]
    fn eval_call_succeed() -> TestResult {
        // true & (false | false | true | false) & (!false) -> true
        let tokens = tokenizer::tokenize("(& T (| F F T F) (^ F))")?;
        let expr = parser::parse(&tokens)?;
        let value = eval(&expr)?;
        assert_eq!(Value::Bool(true), value);
        Ok(())
    }
}
