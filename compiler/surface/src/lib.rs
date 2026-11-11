use syntax::source::Spanned;

mod binder;
mod decl;
mod expr;
mod ident;

pub use binder::{BinderGroup, Plicity};
pub use decl::{Constructor, Declaration, Definition, Inductive};
pub use expr::{ExprKind, Level, Sort, SurfaceExpr};
pub use ident::Ident;

#[derive(Debug, Clone)]
pub struct Module {
    pub declarations: Vec<Spanned<Declaration>>,
}
