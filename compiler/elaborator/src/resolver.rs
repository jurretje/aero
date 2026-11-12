use core::NameId;
use std::collections::HashMap;

use surface::IdentId;

#[derive(Default)]
pub struct NameResolver {
    next_id: usize,
    names: HashMap<String, NameId>,
    occurences: HashMap<IdentId, NameId>,
}
