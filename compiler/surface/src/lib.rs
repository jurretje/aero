use syntax::source::Spanned;

mod binder;
mod decl;
mod expr;
mod ident;

pub use binder::{BinderGroup, Plicity};
pub use decl::{Constructor, Declaration, Inductive, Definition};
pub use expr::{ExprKind, Sort, SurfaceExpr};
pub use ident::Ident;

#[derive(Debug, Clone)]
pub struct Module {
    pub declarations: Vec<Spanned<Declaration>>,
}
