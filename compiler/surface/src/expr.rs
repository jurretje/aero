use crate::{Spanned, binder::BinderGroup, ident::Ident};

pub type SurfaceExpr = Spanned<ExprKind>;

#[derive(Debug, Clone)]
pub enum ExprKind {
    Identifier(Ident),
    Sort(Sort),
    SelfType,
    Application {
        function: Box<SurfaceExpr>,
        argument: Box<SurfaceExpr>,
    },
    Lambda {
        binders: Vec<BinderGroup>,
        body: Box<SurfaceExpr>,
    },
    Pi {
        binders: Vec<BinderGroup>,
        body: Box<SurfaceExpr>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sort {
    Prop,
    Sort(Level),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Level {
    Ident(Ident),
    Zero,
    Succ(Box<Level>),
    Max(Box<Level>, Box<Level>),
}
