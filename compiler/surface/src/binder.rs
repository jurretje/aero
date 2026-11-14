use crate::{IdentId, Expr};

#[derive(Debug, Clone)]
pub struct BinderGroup {
    pub names: Vec<IdentId>,
    pub ty: Option<Expr>,
    pub plicity: Plicity,
}

#[derive(Debug, Clone)]
pub enum Plicity {
    Explicit,
    Implicit,
}
