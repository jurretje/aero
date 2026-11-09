pub enum Expr {
    BoundVar(usize),
    Sort(Level),
    App {
        function: Box<Expr>,
        argument: Box<Expr>,
    },
    Lam {
        info: BinderInfo,
        domain: Box<Expr>,
        body: Box<Expr>
    },
    Pi {
        info: BinderInfo,
        domain: Box<Expr>,
        body: Box<Expr>
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinderInfo {
    Explicit,
    Implicit
}

pub enum Level {
    Zero,
    Succ(Box<Level>),
    Max(Box<Level>, Box<Level>),
    Param,
}
