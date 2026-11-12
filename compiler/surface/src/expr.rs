use std::fmt::Debug;

use crate::{IdentId, QualifiedIdent, Spanned, binder::BinderGroup};

pub type SurfaceExpr = Spanned<ExprKind>;

#[derive(Debug, Clone)]
pub enum ExprKind {
    Identifier(QualifiedIdent),
    Sort(Sort),
    SelfType,
    Application {
        function: Box<SurfaceExpr>,
        argument: Box<SurfaceExpr>,
    },
    Lambda {
        binders: Vec<Spanned<BinderGroup>>,
        body: Box<SurfaceExpr>,
    },
    Pi {
        binders: Vec<Spanned<BinderGroup>>,
        body: Box<SurfaceExpr>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sort {
    Prop,
    Sort(Option<Level>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Level {
    Ident(IdentId),
    Zero,
    Succ(Box<Level>),
    Max(Box<Level>, Box<Level>),
}
