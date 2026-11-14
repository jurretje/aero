use crate::term::{Term, ConstantId};

#[derive(Debug, Clone)]
pub enum Declaration {
    Definition(Definition),
    Inductive(Inductive),
}

#[derive(Debug, Clone)]
pub struct Definition {
    pub name: ConstantId,
    pub ty: Term,
    pub value: Term,
}

#[derive(Debug, Clone)]
pub struct Inductive {
    pub name: ConstantId,
    pub ty: Term,
    pub cons: Vec<Constructor>,
}

#[derive(Debug, Clone)]
pub struct Recursor {
    pub name: ConstantId,
    pub ty: Term,
}

#[derive(Debug, Clone)]
pub struct Constructor {
    pub name: ConstantId,
    pub ty: Term,
}
