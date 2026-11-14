use core::{BinderInfo, Term, ConstantId};

use surface::{IdentId, QualifiedIdent};

#[derive(Debug, Clone)]
pub struct LocalDecl {
    pub ident: IdentId,
    pub ty: Term,
    pub bi: BinderInfo,
}

#[derive(Debug, Clone, Default)]
pub struct LocalContext {
    decls: Vec<LocalDecl>,
    current_inductive: Option<Term>,
}

impl LocalContext {
    pub fn current_inductive(&self) -> &Option<Term> {
        &self.current_inductive
    }

    pub fn set_self(&mut self, name: ConstantId) {
        self.current_inductive = Some(Term::Const(name))
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
