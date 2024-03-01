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
    Operator(parser::Operator),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{b}"),
            Value::Operator(o) => write!(f, "primitive operator: {o}"),
        }
    }
}

fn eval_bool_operands(operands: &[Expr]) -> Result<impl Iterator<Item = bool>, EvalErr> {
    let operands: Vec<Value> = operands
        .iter()
        .map(eval)
        .collect::<Result<Vec<Value>, EvalErr>>()?;
    if operands.iter().any(|arg| !matches!(arg, Value::Bool(_))) {
        return Err(EvalErr::Eval("`operand must be bool".to_string()));
    }
    let operands = operands.into_iter().map(|operand| match operand {
        Value::Bool(b) => b,
        _ => unreachable!(),
    });
    Ok(operands)
}

pub fn eval(expr: &Expr) -> Result<Value, EvalErr> {
    match expr {
        Expr::Bool(b) => Ok(Value::Bool(*b)),
        Expr::Operator(o) => Ok(Value::Operator(*o)),
        Expr::Call(operator, operands) => match eval(operator)? {
            Value::Operator(operator) => match operator {
                parser::Operator::And => {
                    let result = eval_bool_operands(operands)?.fold(true, |acc, b| acc & b);
                    Ok(Value::Bool(result))
                }
                parser::Operator::Or => {
                    let result = eval_bool_operands(operands)?.fold(false, |acc, b| acc | b);
                    Ok(Value::Bool(result))
                }
                parser::Operator::Not => {
                    let operands: Vec<bool> = eval_bool_operands(operands)?.collect();
                    if operands.len() != 1 {
                        return Err(EvalErr::Eval(format!(
                            "the number of arguments of {operator} must be 1"
                        )));
                    }
                    Ok(Value::Bool(!operands[0]))
                }
            },
            operator => Err(EvalErr::Eval(format!("`{operator} is not an operator`"))),
        },
        Expr::If(parser::If { cond, then, other }) => {
            let cond = match eval(cond)? {
                Value::Bool(b) => b,
                value => {
                    return Err(EvalErr::Eval(format!(
                        "condition must be bool, not `{value}`"
                    )))
                }
            };
            eval(if cond { then } else { other })
        }
        Expr::Def(ident, expr) => eval(expr),
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

    #[test]
    fn eval_if_succeed() -> TestResult {
        // if true & true { true } else { false | false } -> true
        let tokens = tokenizer::tokenize("(if (& T T) T (| F F))")?;
        let expr = parser::parse(&tokens)?;
        let value = eval(&expr)?;
        assert_eq!(Value::Bool(true), value);
        Ok(())
    }

    #[test]
    fn eval_if_to_operand_succeed() -> TestResult {
        // if !(true & true) { true } else { false | false } -> true
        let tokens = tokenizer::tokenize("(if (^ (& T T)) & |)")?;
        let expr = parser::parse(&tokens)?;
        let value = eval(&expr)?;
        assert_eq!(Value::Operator(parser::Operator::Or), value);
        Ok(())
    }

    #[test]
    fn eval_if_to_operator_succeed() -> TestResult {
        // if true { true & false } else { true | false }
        let tokens = tokenizer::tokenize("((if T & |) T F)")?;
        let expr = parser::parse(&tokens)?;
        let value = eval(&expr)?;
        assert_eq!(Value::Bool(false), value);
        Ok(())
    }

    #[test]
    fn eval_invalid_operator_fail() -> TestResult {
        let tokens = tokenizer::tokenize("(T T F)")?;
        let expr = parser::parse(&tokens)?;
        match eval(&expr) {
            Err(EvalErr::Eval(_)) => (),
            _ => panic!(),
        }
        Ok(())
    }

    #[test]
    fn eval_def_succeed() -> TestResult {
        let tokens = tokenizer::tokenize("(def myvar (& T T F))")?;
        let expr = parser::parse(&tokens)?;
        let value = eval(&expr)?;
        assert_eq!(Value::Bool(false), value);
        Ok(())
    }
}
