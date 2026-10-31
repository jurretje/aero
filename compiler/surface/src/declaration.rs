use crate::{Expr, Spanned, binder::BinderGroup, ident::Ident};

#[derive(Debug, Clone)]
pub enum Declaration {
    Definition(Definition),
    Inductive(Inductive),
}

#[derive(Debug, Clone)]
pub struct Definition {
    pub name: Ident,
    pub parameters: Vec<Spanned<BinderGroup>>,
    pub ty: Option<Expr>,
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub struct Inductive {
    pub name: Ident,
    pub parameters: Vec<Spanned<BinderGroup>>,
    pub ty: Option<Expr>,
    pub constructors: Vec<Spanned<Constructor>>,
    pub recursor_name: Option<Ident>,
}

#[derive(Debug, Clone)]
pub struct Constructor {
    pub name: Ident,
    pub parameters: Vec<Spanned<BinderGroup>>,
    pub ty: Expr,
}
