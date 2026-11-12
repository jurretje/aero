use core::{BinderInfo, CoreExpr, NameId};

use surface::{IdentId, QualifiedIdent};

#[derive(Debug, Clone)]
pub struct LocalDecl {
    pub ident: IdentId,
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

    pub fn set_self(&mut self, name: NameId) {
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

    pub fn lookup_name(&self, id: &QualifiedIdent) -> Option<(usize, &LocalDecl)> {
        todo!()
        // self.decls
        //     .iter()
        //     .enumerate()
        //     .rev()
        //     .find(|(_, decl)| decl.name.literal() == name)
    }
}
