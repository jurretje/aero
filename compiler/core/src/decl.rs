use crate::expr::{CoreExpr, NameId};

#[derive(Debug, Clone)]
pub enum Declaration {
    Definition(Definition),
    Inductive(Inductive),
}

#[derive(Debug, Clone)]
pub struct Definition {
    pub name: NameId,
    pub ty: CoreExpr,
    pub value: CoreExpr,
}

#[derive(Debug, Clone)]
pub struct Inductive {
    pub name: NameId,
    pub ty: CoreExpr,
    pub cons: Vec<Constructor>,
}

#[derive(Debug, Clone)]
pub struct Recursor {
    pub name: NameId,
    pub ty: CoreExpr,
}

#[derive(Debug, Clone)]
pub struct Constructor {
    pub name: NameId,
    pub ty: CoreExpr,
}
