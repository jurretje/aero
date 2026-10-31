use syntax::source::Span;

mod binder;
mod declaration;
mod expression;
mod ident;

pub use binder::{BinderGroup, Plicity};
pub use declaration::{Constructor, Declaration, Definition, Inductive};
pub use expression::{Expr, ExprKind, Sort};
pub use ident::Ident;

#[derive(Debug, Clone)]
pub struct Module {
    pub declarations: Vec<Spanned<Declaration>>,
}

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(node: T, span: Span) -> Self {
        Self { node, span }
    }

    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Spanned<U> {
        Spanned {
            node: f(self.node),
            span: self.span,
        }
    }
}
