use std::collections::HashMap;

use source::map::TextRef;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExprId(pub usize);

#[derive(Debug, Clone)]
pub enum Literal {
    Bootstrap(char),
    Source(TextRef),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CharClass {
    Alphabetic,
    Alphanumeric,
    Whitespace,
    Digit,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Class(CharClass),
    Seq(Vec<ExprId>),
    Alt(Vec<ExprId>),
    Opt(ExprId),
    Star(ExprId),
    Plus(ExprId),
    Ref(RuleId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RuleId(pub usize);

#[derive(Debug, Clone)]
pub struct Rule {
    expression: ExprId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RuleName {
    Bootstrap(&'static str),
    Source(TextRef),
}

#[derive(Debug, Default, Clone)]
pub struct RuleRegistry {
    rules: HashMap<RuleId, Rule>,
    names: HashMap<RuleName, RuleId>,
    exprs: Vec<Expr>,
    next_id: usize,
}

impl RuleRegistry {
    pub fn declare_rule(&mut self) -> RuleId {
        let id = RuleId(self.next_id);
        self.next_id += 1;
        id
    }

    pub fn define_rule(&mut self, id: RuleId, expression: ExprId) -> Option<RuleId> {
        self.rules.insert(id, Rule { expression });

        Some(id)
    }

    fn add_expr(&mut self, expr: Expr) -> ExprId {
        let id = ExprId(self.exprs.len());
        self.exprs.push(expr);
        id
    }
}

impl RuleRegistry {
    pub fn lit(&mut self, literal: Literal) -> ExprId {
        self.add_expr(Expr::Literal(literal))
    }

    pub fn str(&mut self, s: &'static str) -> ExprId {
        let exprs: Vec<_> = s.chars().map(|c| self.lit(Literal::Bootstrap(c))).collect();
        self.seq(exprs)
    }

    pub fn class(&mut self, class: CharClass) -> ExprId {
        self.add_expr(Expr::Class(class))
    }

    pub fn seq(&mut self, exprs: impl IntoIterator<Item = ExprId>) -> ExprId {
        self.add_expr(Expr::Seq(exprs.into_iter().collect()))
    }

    pub fn alt(&mut self, exprs: impl IntoIterator<Item = ExprId>) -> ExprId {
        self.add_expr(Expr::Alt(exprs.into_iter().collect()))
    }

    pub fn opt(&mut self, expr: ExprId) -> ExprId {
        self.add_expr(Expr::Opt(expr))
    }

    pub fn star(&mut self, expr: ExprId) -> ExprId {
        self.add_expr(Expr::Star(expr))
    }

    pub fn plus(&mut self, expr: ExprId) -> ExprId {
        self.add_expr(Expr::Plus(expr))
    }

    pub fn reference(&mut self, rule: RuleId) -> ExprId {
        self.add_expr(Expr::Ref(rule))
    }
}
