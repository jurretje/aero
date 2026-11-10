use crate::{SurfaceExpr, ident::Ident};

#[derive(Debug, Clone)]
pub struct BinderGroup {
    pub names: Vec<Ident>,
    pub ty: Option<SurfaceExpr>,
    pub plicity: Plicity,
}

#[derive(Debug, Clone)]
pub enum Plicity {
    Explicit,
    Implicit,
}
