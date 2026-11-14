use crate::{BinderGroup, Expr, IdentId, Spanned};

#[derive(Debug, Clone)]
pub enum Declaration {
    Definition(Definition),
    Inductive(Inductive),
    Module(IdentId),
}

#[derive(Debug, Clone)]
pub struct Definition {
    pub name: IdentId,
    pub parameters: Vec<Spanned<BinderGroup>>,
    pub ty: Option<Expr>,
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub struct Inductive {
    pub name: IdentId,
    pub parameters: Vec<Spanned<BinderGroup>>,
    pub ty: Option<Expr>,
    pub cons: Vec<Spanned<Constructor>>,
    pub recursor_name: Option<IdentId>,
}

#[derive(Debug, Clone)]
pub struct Constructor {
    pub name: IdentId,
    pub parameters: Vec<Spanned<BinderGroup>>,
    pub ty: Expr,
}
