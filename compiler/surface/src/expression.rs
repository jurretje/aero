use crate::{binder::BinderGroup, ident::Ident};

#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(Ident),
    Sort(Sort),
    Application {
        function: Box<Expr>,
        argument: Box<Expr>,
    },
    Lambda {
        binders: Vec<BinderGroup>,
        body: Box<Expr>,
    },
    Pi {
        binders: Vec<BinderGroup>,
        body: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sort {
    Prop,
    Sort(u32),
}
