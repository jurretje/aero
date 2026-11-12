use core::NameId;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Environment {
    declarations: HashMap<NameId, core::Declaration>,
}

impl Environment {
    pub fn contains(&self, id: &NameId) -> bool {
        todo!()
    }
}
