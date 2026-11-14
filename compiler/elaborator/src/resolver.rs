use core::ConstantId;
use std::collections::HashMap;

use surface::IdentId;

#[derive(Default)]
pub struct NameResolver {
    next_id: usize,
    names: HashMap<String, ConstantId>,
    occurences: HashMap<IdentId, ConstantId>,
}
