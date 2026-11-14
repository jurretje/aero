mod decl;
mod term;

pub use decl::{Constructor, Declaration, Definition, Inductive, Recursor};
pub use term::{BinderInfo, Term, Level, ConstantId};

#[derive(Debug, Clone)]
pub struct Module {
    pub decls: Vec<Declaration>,
}
