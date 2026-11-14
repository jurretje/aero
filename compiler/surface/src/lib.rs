mod binder;
mod decl;
mod expr;
mod ident;

pub use binder::{BinderGroup, Plicity};
pub use decl::{Constructor, Declaration, Definition, Inductive};
pub use expr::{Expr, ExprKind, Level, Sort};
pub use ident::{IdentId, QualifiedIdent};
use source::span::Spanned;

#[derive(Debug, Clone)]
pub struct Module {
    pub declarations: Vec<Spanned<Declaration>>,
}
