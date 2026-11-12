use crate::{BinderGroup, IdentId, Spanned, SurfaceExpr};

#[derive(Debug, Clone)]
pub enum Declaration {
    Definition(Definition),
    Inductive(Inductive),
}

#[derive(Debug, Clone)]
pub struct Definition {
    pub name: IdentId,
    pub parameters: Vec<Spanned<BinderGroup>>,
    pub ty: Option<SurfaceExpr>,
    pub value: SurfaceExpr,
}

#[derive(Debug, Clone)]
pub struct Inductive {
    pub name: IdentId,
    pub parameters: Vec<Spanned<BinderGroup>>,
    pub ty: Option<SurfaceExpr>,
    pub cons: Vec<Spanned<Constructor>>,
    pub recursor_name: Option<IdentId>,
}

#[derive(Debug, Clone)]
pub struct Constructor {
    pub name: IdentId,
    pub parameters: Vec<Spanned<BinderGroup>>,
    pub ty: SurfaceExpr,
}
