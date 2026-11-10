use crate::expr::{CoreExpr, Name};

#[derive(Debug, Clone)]
pub enum Declaration {
    Definition(Definition),
    Inductive(Inductive),
}

#[derive(Debug, Clone)]
pub struct Definition {
    pub name: Name,
    pub ty: CoreExpr,
    pub value: CoreExpr,
}

#[derive(Debug, Clone)]
pub struct Inductive {
    pub name: Name,
    pub ty: CoreExpr,
    pub cons: Vec<Constructor>,
}

#[derive(Debug, Clone)]
pub struct Recursor {
    pub name: Name,
    pub ty: CoreExpr,
}

#[derive(Debug, Clone)]
pub struct Constructor {
    pub name: Name,
    pub ty: CoreExpr,
}
