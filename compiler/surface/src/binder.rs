use crate::{IdentId, SurfaceExpr};

#[derive(Debug, Clone)]
pub struct BinderGroup {
    pub names: Vec<IdentId>,
    pub ty: Option<SurfaceExpr>,
    pub plicity: Plicity,
}

#[derive(Debug, Clone)]
pub enum Plicity {
    Explicit,
    Implicit,
}
