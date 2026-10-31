use crate::declaration::Declaration;

pub mod binder;
pub mod declaration;
pub mod expression;
pub mod ident;

#[derive(Debug, Clone)]
pub struct Module {
    pub declarations: Vec<Declaration>,
}
