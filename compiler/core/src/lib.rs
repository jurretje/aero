mod decl;
mod expr;

pub use decl::{Constructor, Declaration, Definition, Inductive, Recursor};
pub use expr::{BinderInfo, CoreExpr, Level, Name};

#[derive(Debug, Clone)]
pub struct Module {
    pub decls: Vec<Declaration>,
}
