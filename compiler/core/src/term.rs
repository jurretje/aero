#[derive(Debug, Clone)]
pub enum Term {
    Var(usize),
    Sort(Level),
    Const(ConstantId),
    App {
        function: Box<Term>,
        argument: Box<Term>,
    },
    Lam {
        info: BinderInfo,
        domain: Box<Term>,
        body: Box<Term>,
    },
    Pi {
        info: BinderInfo,
        domain: Box<Term>,
        body: Box<Term>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinderInfo {
    Explicit,
    Implicit,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Level {
    Zero,
    Succ(Box<Level>),
    Max(Box<Level>, Box<Level>),
    Param(LevelParamId),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LevelParamId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstantId(pub usize);
