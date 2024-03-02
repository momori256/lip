use crate::evaluator::Value;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct Environment {
    data: HashMap<String, Value>,
}

impl Environment {
    pub fn new(data: HashMap<String, Value>) -> Self {
        Self { data }
    }

    pub fn add(&mut self, var: String, value: Value) {
        self.data.insert(var, value);
    }

    pub fn get(&self, var: &str) -> Option<&Value> {
        self.data.get(var)
    }

    pub fn extend(&mut self, other: HashMap<String, Value>) {
        self.data.extend(other)
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
}
