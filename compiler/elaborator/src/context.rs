use core::{BinderInfo, CoreExpr, Name};

use surface::Ident;

#[derive(Debug, Clone)]
pub struct LocalDecl {
    pub name: Ident,
    pub ty: CoreExpr,
    pub bi: BinderInfo,
}

#[derive(Debug, Clone, Default)]
pub struct LocalContext {
    decls: Vec<LocalDecl>,
    current_inductive: Option<CoreExpr>,
}

impl LocalContext {
    pub fn current_inductive(&self) -> &Option<CoreExpr> {
        &self.current_inductive
    }

    pub fn set_self(&mut self, name: Name) {
        self.current_inductive = Some(CoreExpr::Const(name))
    }

    pub fn reset_self(&mut self) {
        self.current_inductive = None;
    }

    pub fn push(&mut self, decl: LocalDecl) {
        self.decls.push(decl);
    }

    pub fn checkpoint(&self) -> usize {
        self.decls.len()
    }

    pub fn truncate(&mut self, checkpoint: usize) {
        self.decls.truncate(checkpoint);
    }

    pub fn pop(&mut self, name: &str) -> Option<(usize, &LocalDecl)> {
        self.decls
            .iter()
            .enumerate()
            .rev()
            .find(|(_, decl)| decl.name.literal() == name)
            .map(|(position, decl)| {
                let index = self.decls.len() - 1 - position;
                (index, decl)
            })
    }
    pub fn lookup_name(&self, name: &str) -> Option<(usize, &LocalDecl)> {
        self.decls
            .iter()
            .enumerate()
            .rev()
            .find(|(_, decl)| decl.name.literal() == name)
    }
}
