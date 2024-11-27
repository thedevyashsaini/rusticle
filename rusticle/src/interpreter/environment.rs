use std::collections::HashMap;
use crate::{parser::ast::Object, utils::token::Token};

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Option<Object> {
        self.values.get(&name.lexeme).cloned()
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), String> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        } else {
            Err(format!("Undefined variable '{}'.", name.lexeme))
        }
    }
}