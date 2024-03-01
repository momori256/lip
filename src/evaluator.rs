use std::collections::HashMap;

use crate::environment::Environment;
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
    Bool(bool),
    Operator(parser::Operator),
    Lambda(Vec<String>, Expr),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{b}"),
            Value::Operator(o) => write!(f, "primitive operator: {o}"),
            Value::Lambda(args, expr) => write!(f, "lambda: {args:?} -> {expr:?}"),
        }
    }
}

fn eval_bool_operands(
    operands: &[Expr],
    env: &mut Environment,
) -> Result<impl Iterator<Item = bool>, EvalErr> {
    let operands: Vec<Value> = operands
        .iter()
        .map(|operand| eval(operand, env))
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

pub fn eval(expr: &Expr, env: &mut Environment) -> Result<Value, EvalErr> {
    match expr {
        Expr::Bool(b) => Ok(Value::Bool(*b)),
        Expr::Operator(o) => Ok(Value::Operator(*o)),
        Expr::Call(operator, operands) => match eval(operator, env)? {
            Value::Operator(operator) => match operator {
                parser::Operator::And => {
                    let result = eval_bool_operands(operands, env)?.fold(true, |acc, b| acc & b);
                    Ok(Value::Bool(result))
                }
                parser::Operator::Or => {
                    let result = eval_bool_operands(operands, env)?.fold(false, |acc, b| acc | b);
                    Ok(Value::Bool(result))
                }
                parser::Operator::Not => {
                    let operands: Vec<bool> = eval_bool_operands(operands, env)?.collect();
                    if operands.len() != 1 {
                        return Err(EvalErr::Eval(format!(
                            "the number of arguments of {operator} must be 1"
                        )));
                    }
                    Ok(Value::Bool(!operands[0]))
                }
            },
            Value::Lambda(args, expr) => {
                if args.len() != operands.len() {
                    return Err(EvalErr::Eval(format!(
                        "the number of arguments ({}) is not the same as that of parameters ({})",
                        args.len(),
                        operands.len()
                    )));
                }
                let operands: Vec<Value> = operands
                    .into_iter()
                    .map(|operand| eval(operand, env))
                    .collect::<Result<_, EvalErr>>()?;
                let data: HashMap<String, Value> = args.into_iter().zip(operands).collect();
                let mut inner = Environment::new(data, Some(&env));
                eval(&expr, &mut inner)
            }
            operator => Err(EvalErr::Eval(format!("`{operator} is not an operator`"))),
        },
        Expr::If(parser::If { cond, then, other }) => {
            let cond = match eval(cond, env)? {
                Value::Bool(b) => b,
                value => {
                    return Err(EvalErr::Eval(format!(
                        "condition must be bool, not `{value}`"
                    )))
                }
            };
            eval(if cond { then } else { other }, env)
        }
        Expr::Def(ident, expr) => {
            let result = eval(expr, env)?;
            env.add(ident.to_string(), result.clone());
            Ok(result)
        }
        Expr::Lambda(args, expr) => Ok(Value::Lambda(args.clone(), (**expr).clone())),
        Expr::Ident(ident) => {
            if let Some(value) = env.get(ident) {
                Ok(value.clone())
            } else {
                Err(EvalErr::Eval(format!("`{ident}` is not defined")))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parser, tokenizer};

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    fn and(exprs: Vec<Expr>) -> Expr {
        call(Expr::Operator(parser::Operator::And), exprs)
    }

    fn call(operator: Expr, operands: Vec<Expr>) -> Expr {
        Expr::Call(Box::new(operator), operands)
    }

    fn ident(ident: &str) -> Expr {
        Expr::Ident(ident.to_string())
    }

    #[test]
    fn eval_bool_succeed() -> TestResult {
        let tokens = tokenizer::tokenize("T")?;
        let expr = parser::parse(&tokens)?;
        let value = eval(&expr, &mut Environment::default())?;
        assert_eq!(Value::Bool(true), value);
        Ok(())
    }

    #[test]
    fn eval_call_succeed() -> TestResult {
        // true & (false | false | true | false) & (!false) -> true
        let tokens = tokenizer::tokenize("(& T (| F F T F) (^ F))")?;
        let expr = parser::parse(&tokens)?;
        let value = eval(&expr, &mut Environment::default())?;
        assert_eq!(Value::Bool(true), value);
        Ok(())
    }

    #[test]
    fn eval_if_succeed() -> TestResult {
        // if true & true { true } else { false | false } -> true
        let tokens = tokenizer::tokenize("(if (& T T) T (| F F))")?;
        let expr = parser::parse(&tokens)?;
        let value = eval(&expr, &mut Environment::default())?;
        assert_eq!(Value::Bool(true), value);
        Ok(())
    }

    #[test]
    fn eval_if_to_operand_succeed() -> TestResult {
        // if !(true & true) { true } else { false | false } -> true
        let tokens = tokenizer::tokenize("(if (^ (& T T)) & |)")?;
        let expr = parser::parse(&tokens)?;
        let value = eval(&expr, &mut Environment::default())?;
        assert_eq!(Value::Operator(parser::Operator::Or), value);
        Ok(())
    }

    #[test]
    fn eval_if_to_operator_succeed() -> TestResult {
        // if true { true & false } else { true | false }
        let tokens = tokenizer::tokenize("((if T & |) T F)")?;
        let expr = parser::parse(&tokens)?;
        let value = eval(&expr, &mut Environment::default())?;
        assert_eq!(Value::Bool(false), value);
        Ok(())
    }

    #[test]
    fn eval_invalid_operator_fail() -> TestResult {
        let tokens = tokenizer::tokenize("(T T F)")?;
        let expr = parser::parse(&tokens)?;
        match eval(&expr, &mut Environment::default()) {
            Err(EvalErr::Eval(_)) => (),
            _ => panic!(),
        }
        Ok(())
    }

    #[test]
    fn eval_def_succeed() -> TestResult {
        let mut env = Environment::default();
        {
            let tokens = tokenizer::tokenize("(def myvar (& T T F))")?;
            let expr = parser::parse(&tokens)?;
            let value = eval(&expr, &mut env)?;
            assert_eq!(Value::Bool(false), value);
        }
        {
            let tokens = tokenizer::tokenize("myvar")?;
            let expr = parser::parse(&tokens)?;
            let value = eval(&expr, &mut env)?;
            assert_eq!(Value::Bool(false), value);
        }
        {
            let tokens = tokenizer::tokenize("(def myvar (& T T T))")?;
            let expr = parser::parse(&tokens)?;
            let value = eval(&expr, &mut env)?;
            assert_eq!(Value::Bool(true), value);
        }
        {
            let tokens = tokenizer::tokenize("myvar")?;
            let expr = parser::parse(&tokens)?;
            let value = eval(&expr, &mut env)?;
            assert_eq!(Value::Bool(true), value);
        }
        Ok(())
    }

    #[test]
    fn eval_lambda_succeed() -> TestResult {
        let tokens = tokenizer::tokenize("(lambda (a b) (& a b T))")?;
        let expr = parser::parse(&tokens)?;
        let value = eval(&expr, &mut Environment::default())?;
        assert_eq!(
            Value::Lambda(
                vec!["a".to_string(), "b".to_string()],
                and(vec![ident("a"), ident("b"), Expr::Bool(true)])
            ),
            value
        );
        Ok(())
    }

    #[test]
    fn eval_call_lambda_succeed() -> TestResult {
        let tokens = tokenizer::tokenize("((lambda (a b c) (| a b c)) F F T)")?;
        let expr = parser::parse(&tokens)?;
        let value = eval(&expr, &mut Environment::default())?;
        assert_eq!(Value::Bool(true), value);
        Ok(())
    }

    #[test]
    fn eval_def_lambda_succeed() -> TestResult {
        let mut env = Environment::default();
        {
            let tokens = tokenizer::tokenize("(def nand (lambda (a b) (^ (& a b))))")?;
            let expr = parser::parse(&tokens)?;
            let value = eval(&expr, &mut env)?;
            assert!(matches!(value, Value::Lambda(_, _)));
        }
        {
            let tokens = tokenizer::tokenize("(nand T T)")?;
            let expr = parser::parse(&tokens)?;
            let value = eval(&expr, &mut env)?;
            assert_eq!(Value::Bool(false), value);
        }
        Ok(())
    }
}
