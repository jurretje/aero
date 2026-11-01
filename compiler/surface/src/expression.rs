use crate::{Spanned, binder::BinderGroup, ident::Ident};

pub type Expr = Spanned<ExprKind>;

#[derive(Debug, Clone)]
pub enum ExprKind {
    Identifier(Ident),
    Sort(Sort),
    SelfType,
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
