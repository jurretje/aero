use crate::{Spanned, SurfaceExpr, binder::BinderGroup, ident::Ident};

#[derive(Debug, Clone)]
pub enum Declaration {
    Definition(Definition),
    Inductive(Inductive),
}

#[derive(Debug, Clone)]
pub struct Definition {
    pub name: Ident,
    pub parameters: Vec<Spanned<BinderGroup>>,
    pub ty: Option<SurfaceExpr>,
    pub value: SurfaceExpr,
}

#[derive(Debug, Clone)]
pub struct Inductive {
    pub name: Ident,
    pub parameters: Vec<Spanned<BinderGroup>>,
    pub ty: Option<SurfaceExpr>,
    pub cons: Vec<Spanned<Constructor>>,
    pub recursor_name: Option<Ident>,
}

#[derive(Debug, Clone)]
pub struct Constructor {
    pub name: Ident,
    pub parameters: Vec<Spanned<BinderGroup>>,
    pub ty: SurfaceExpr,
}
