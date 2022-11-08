use crate::object::Object;
use crate::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Environment contains all stored value during the program execution
/// A variable is bounded to an executed value in runtime and the value can be retried using variable name
/// `let a = "before";`
///
/// Environment takes ownership of all the variables declared and only provide an reference ptr to the variable upon demand
#[derive(Debug, Default)]
pub struct Environment {
    values: HashMap<String, Rc<RefCell<Object>>>,
}

impl Environment {
    /// Insert a declared variable to environment to store and can be retrieved later
    pub fn define(&mut self, name: &str, value: Object) -> Rc<RefCell<Object>> {
        // TODO: Add error handler which checks if the variable exist and is mutable before setting the value again.
        let value = Rc::new(RefCell::new(value));
        let ret_value = value.clone();
        self.values.insert(name.to_string(), value);
        ret_value
    }

    pub fn assign(&mut self, token: &Token, value: Object) -> Option<Rc<RefCell<Object>>> {
        let name = &token.lexeme;
        let value = Rc::new(RefCell::new(value));
        let ret_value = value.clone();
        match self.values.get(name) {
            Some(_) => {
                self.values.insert(name.to_string(), value);
                Some(ret_value)
            }
            None => None,
        }
    }

    /// Get a the `Object` value of a stored variable.
    /// returns `None` if the variable doesn't exist in the environment and should be treated as error
    pub fn get(&self, token: &Token) -> Option<Rc<RefCell<Object>>> {
        self.values.get(&token.lexeme).cloned()
    }
}
