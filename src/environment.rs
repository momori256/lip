use crate::evaluator::Value;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Environment<'a> {
    data: HashMap<String, Value>,
    outer: Option<&'a Environment<'a>>,
}

impl<'a> Environment<'a> {
    pub fn new(data: HashMap<String, Value>, outer: Option<&'a Environment>) -> Self {
        Self { data, outer }
    }

    pub fn add(&mut self, var: String, value: Value) {
        self.data.insert(var, value);
    }

    pub fn get(&self, var: &str) -> Option<&Value> {
        if let Some(value) = self.data.get(var) {
            return Some(value);
        }
        if let Some(outer) = &self.outer {
            return outer.get(var);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn environment_add_and_get_succeed() {
        let mut env = Environment::default();
        assert!(env.get("myvar").is_none());
        env.add("myvar".to_string(), Value::Bool(false));
        assert_eq!(Value::Bool(false), *env.get("myvar").unwrap());
    }

    #[test]
    fn environment_layered_succeed() {
        let env_outer =
            Environment::new(HashMap::from([("x".to_string(), Value::Bool(true))]), None);
        let env = Environment::new(HashMap::new(), Some(&env_outer));
        assert_eq!(Value::Bool(true), *env.get("x").unwrap());
    }
}
