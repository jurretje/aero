use core::ConstantId;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Environment {
    declarations: HashMap<ConstantId, core::Declaration>,
}

impl Environment {
    pub fn contains(&self, id: &ConstantId) -> bool {
        todo!()
    }
}
