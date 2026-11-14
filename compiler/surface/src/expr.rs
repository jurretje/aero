use std::fmt::Debug;

use crate::{IdentId, QualifiedIdent, Spanned, binder::BinderGroup};

pub type Expr = Spanned<ExprKind>;

#[derive(Debug, Clone)]
pub enum ExprKind {
    Identifier(QualifiedIdent),
    Sort(Sort),
    SelfType,
    Application {
        function: Box<Expr>,
        argument: Box<Expr>,
    },
    Lambda {
        binders: Vec<Spanned<BinderGroup>>,
        body: Box<Expr>,
    },
    Pi {
        binders: Vec<Spanned<BinderGroup>>,
        body: Box<Expr>,
    },
    Hole
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
