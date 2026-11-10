use surface::Ident;

#[derive(Debug, Clone)]
pub enum CoreExpr {
    BoundVar(usize),
    Sort(Level),
    Const(Name),
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
    Param(Name),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name(String);

impl Name {
    pub fn new(name: String) -> Self {
        Self(name)
    }

    pub fn literal(&self) -> &str {
        &self.0
    }
}

impl From<&Ident> for Name {
    fn from(value: &Ident) -> Self {
        Self(value.literal().to_owned())
    }
}
