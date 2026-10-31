use crate::{binder::BinderGroup, expression::Expr, ident::Ident};

#[derive(Debug, Clone)]
pub enum Declaration {
    Definition(Definition),
    Inductive(Inductive),
}

#[derive(Debug, Clone)]
pub struct Definition {
    pub name: Ident,
    pub parameters: Vec<BinderGroup>,
    pub ty: Option<Expr>,
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub struct Inductive {
    pub name: Ident,
    pub parameters: Vec<BinderGroup>,
    pub ty: Option<Expr>,
    pub constructors: Vec<Constructor>,
    pub recursor_name: Option<Ident>,
}

#[derive(Debug, Clone)]
pub struct Constructor {
    pub name: Ident,
    pub ty: Expr,
}
