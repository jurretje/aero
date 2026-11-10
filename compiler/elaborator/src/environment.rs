use core::{Declaration, Name};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Environment {
    decls: HashMap<Name, Declaration>,
}

impl Environment {
    pub fn contains(&self, name: &Name) -> bool {
        self.decls.contains_key(name)
    }

    pub fn get(&self, name: &Name) -> Option<&Declaration> {
        self.decls.get(name)
    }

    pub fn insert(&mut self, name: &Name, declaration: Declaration) {
        self.decls.insert(name.clone(), declaration);
    }
}
