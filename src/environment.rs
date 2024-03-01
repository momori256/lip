use crate::evaluator::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Environment {
    data: HashMap<String, Value>,
}

impl Environment {
    pub fn new(data: HashMap<String, Value>) -> Self {
        Self { data }
    }

    pub fn add(&mut self, var: String, value: Value) {
        if let Some(e) = self.data.get_mut(&var) {
            *e = value;
            return;
        }
        self.data.entry(var).or_insert(value);
    }

    pub fn get(&self, var: &str) -> Option<&Value> {
        self.data.get(var)
    }
}

impl std::default::Default for Environment {
    fn default() -> Self {
        let data = HashMap::new();
        Self { data }
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
