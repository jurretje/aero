use crate::{expression::Expr, ident::Ident};

#[derive(Debug, Clone)]
pub struct BinderGroup {
    pub names: Vec<Ident>,
    pub ty: Option<Expr>,
    pub plicity: Plicity,
}

#[derive(Debug, Clone)]
pub enum Plicity {
    Explicit,
    Implicit,
}
