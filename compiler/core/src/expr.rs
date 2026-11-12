#[derive(Debug, Clone)]
pub enum CoreExpr {
    BoundVar(usize),
    Sort(Level),
    Const(NameId),
    App {
        function: Box<CoreExpr>,
        argument: Box<CoreExpr>,
    },
    Lam {
        info: BinderInfo,
        domain: Box<CoreExpr>,
        body: Box<CoreExpr>,
    },
    Pi {
        info: BinderInfo,
        domain: Box<CoreExpr>,
        body: Box<CoreExpr>,
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
    Param(NameId),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NameId(pub usize);
