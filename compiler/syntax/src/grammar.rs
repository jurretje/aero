use serde::Serialize;

use crate::token::TokenKind;

pub struct Grammar {
    pub rules: Box<[Rule]>,
    pub starting_rule: RuleName,
}

impl Grammar {
    pub fn get(&self, name: RuleName) -> &Rule {
        let r = &self.rules[name as usize];
        if r.name != name {
            unreachable!()
        }
        r
    }
}

pub struct Rule {
    pub name: RuleName,
    pub kind: RuleKind,
}

#[derive(Debug)]
pub enum RuleKind {
    Token(TokenKind),
    Ref(RuleName),

    Seq(Vec<Self>),
    Alt(Vec<Self>),

    Question(Box<Self>),
    Plus(Box<Self>),
    Star(Box<Self>),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize)]
#[repr(usize)]
pub enum RuleName {
    Env,
    Ind,
    Constructor,
    Const,
    Params,
    ExplicitSection,
    ImplicitSection,
    TypeSpec,
    Expr,
    AppExpr,
    AtomicExpr,
    ParenExpr,
    Lambda,
    Forall,
    QualifiedIdent,
}

impl RuleName {
    pub const ALL: [Self; 15] = [
        Self::Env,
        Self::Ind,
        Self::Constructor,
        Self::Const,
        Self::Params,
        Self::ExplicitSection,
        Self::ImplicitSection,
        Self::TypeSpec,
        Self::Expr,
        Self::AppExpr,
        Self::AtomicExpr,
        Self::ParenExpr,
        Self::Lambda,
        Self::Forall,
        Self::QualifiedIdent,
    ];
}

pub fn build_grammar() -> Grammar {
    use RuleKind as Rk;

    Grammar {
        starting_rule: RuleName::Env,
        rules: Box::new([
            Rule {
                name: RuleName::Env,
                kind: Rk::Star(Box::new(Rk::Alt(vec![
                    Rk::Ref(RuleName::Ind),
                    Rk::Ref(RuleName::Const),
                ]))),
            },
            Rule {
                name: RuleName::Ind,
                kind: Rk::Seq(vec![
                    Rk::Token(TokenKind::Ind),
                    Rk::Token(TokenKind::Ident),
                    Rk::Ref(RuleName::Params),
                    Rk::Ref(RuleName::TypeSpec),
                    Rk::Token(TokenKind::Assign),
                    Rk::Star(Box::new(Rk::Ref(RuleName::Constructor))),
                    Rk::Token(TokenKind::Elim),
                    Rk::Token(TokenKind::Ident),
                ]),
            },
            Rule {
                name: RuleName::Constructor,
                kind: Rk::Seq(vec![
                    Rk::Token(TokenKind::Pipe),
                    Rk::Token(TokenKind::Ident),
                    Rk::Ref(RuleName::TypeSpec),
                ]),
            },
            Rule {
                name: RuleName::Const,
                kind: Rk::Seq(vec![
                    Rk::Token(TokenKind::Def),
                    Rk::Token(TokenKind::Ident),
                    Rk::Ref(RuleName::Params),
                    Rk::Ref(RuleName::TypeSpec),
                    Rk::Token(TokenKind::Assign),
                    Rk::Ref(RuleName::Expr),
                ]),
            },
            Rule {
                name: RuleName::Params,
                kind: Rk::Star(Box::new(Rk::Alt(vec![
                    Rk::Ref(RuleName::ExplicitSection),
                    Rk::Ref(RuleName::ImplicitSection),
                ]))),
            },
            Rule {
                name: RuleName::ExplicitSection,
                kind: Rk::Seq(vec![
                    Rk::Token(TokenKind::OpenParen),
                    Rk::Plus(Box::new(Rk::Token(TokenKind::Ident))),
                    Rk::Ref(RuleName::TypeSpec),
                    Rk::Token(TokenKind::CloseParen),
                ]),
            },
            Rule {
                name: RuleName::ImplicitSection,
                kind: Rk::Seq(vec![
                    Rk::Token(TokenKind::OpenBrace),
                    Rk::Plus(Box::new(Rk::Token(TokenKind::Ident))),
                    Rk::Ref(RuleName::TypeSpec),
                    Rk::Token(TokenKind::CloseBrace),
                ]),
            },
            Rule {
                name: RuleName::TypeSpec,
                kind: Rk::Seq(vec![Rk::Token(TokenKind::Colon), Rk::Ref(RuleName::Expr)]),
            },
            Rule {
                name: RuleName::Expr,
                kind: Rk::Seq(vec![
                    Rk::Ref(RuleName::AppExpr),
                    Rk::Question(Box::new(Rk::Seq(vec![
                        Rk::Token(TokenKind::Arrow),
                        Rk::Ref(RuleName::Expr),
                    ]))),
                ]),
            },
            Rule {
                name: RuleName::AppExpr,
                kind: Rk::Seq(vec![
                    Rk::Ref(RuleName::AtomicExpr),
                    Rk::Star(Box::new(Rk::Ref(RuleName::AtomicExpr))),
                ]),
            },
            Rule {
                name: RuleName::AtomicExpr,
                kind: Rk::Alt(vec![
                    Rk::Ref(RuleName::QualifiedIdent),
                    Rk::Ref(RuleName::ParenExpr),
                    Rk::Ref(RuleName::Lambda),
                    Rk::Ref(RuleName::Forall),
                    Rk::Token(TokenKind::Prop),
                    Rk::Token(TokenKind::Type),
                ]),
            },
            Rule {
                name: RuleName::ParenExpr,
                kind: Rk::Seq(vec![
                    Rk::Token(TokenKind::OpenParen),
                    Rk::Ref(RuleName::Expr),
                    Rk::Token(TokenKind::CloseParen),
                ]),
            },
            Rule {
                name: RuleName::Lambda,
                kind: Rk::Seq(vec![
                    Rk::Token(TokenKind::Fn),
                    Rk::Token(TokenKind::Ident),
                    Rk::Ref(RuleName::TypeSpec),
                    Rk::Token(TokenKind::FatArrow),
                    Rk::Ref(RuleName::Expr),
                ]),
            },
            Rule {
                name: RuleName::Forall,
                kind: Rk::Seq(vec![
                    Rk::Token(TokenKind::Forall),
                    Rk::Token(TokenKind::Ident),
                    Rk::Ref(RuleName::TypeSpec),
                    Rk::Token(TokenKind::Comma),
                    Rk::Ref(RuleName::Expr),
                ]),
            },
            Rule {
                name: RuleName::QualifiedIdent,
                kind: Rk::Seq(vec![
                    Rk::Token(TokenKind::Ident),
                    Rk::Star(Box::new(Rk::Seq(vec![
                        Rk::Token(TokenKind::Dot),
                        Rk::Token(TokenKind::Ident),
                    ]))),
                ]),
            },
        ]),
    }
}