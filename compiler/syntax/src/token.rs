use std::fmt::Debug;

use crate::{source::Span, style::Style};

#[derive(PartialEq, Eq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TokenKind {
    Ind,
    Elim,
    Def,
    Prop,
    Type,
    Assign,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Ident,
    Colon,
    Pipe,
    Arrow,
    FatArrow,
    Fn,
    Forall,
    Comma,
    Dot,
}

impl TokenKind {
    pub fn style(self) -> Style {
        use TokenKind::*;

        match self {
            Ind | Elim | Def | Fn | Forall => Style::Keyword,
            Prop | Type => Style::Type,
            Ident => Style::Identifier,
            Assign | Arrow | FatArrow => Style::Operator,
            OpenParen | CloseParen | OpenBrace | CloseBrace | Colon | Pipe | Comma | Dot => {
                Style::Punctuation
            }
        }
    }

    pub fn matches(self, input: &str) -> Option<(TokenKind, usize)> {
        match self.literal() {
            Some(text) => input.starts_with(text).then_some((self, text.len())),
            None => self.match_ident(input),
        }
    }

    fn match_ident(self, input: &str) -> Option<(TokenKind, usize)> {
        let mut chars = input.char_indices();

        let (_, first) = match chars.next() {
            Some(v) => v,
            None => return None,
        };

        if !first.is_alphabetic() && first != '_' {
            return None;
        }

        let mut last_valid = first.len_utf8();

        for (i, c) in chars {
            if c.is_alphanumeric() || c == '_' {
                last_valid = i + c.len_utf8();
            } else {
                break;
            }
        }

        Some((TokenKind::Ident, last_valid))
    }

    pub fn all() -> &'static [TokenKind] {
        use TokenKind::*;
        &[
            Ind, Elim, Def, Prop, Type, Assign, OpenParen, CloseParen, OpenBrace, CloseBrace,
            Colon, Pipe, Arrow, FatArrow, Fn, Forall, Comma, Ident, Dot,
        ]
    }

    pub fn literal(&self) -> Option<&'static str> {
        use TokenKind::*;
        Some(match self {
            Ind => "ind",
            Elim => "elim",
            Def => "def",
            Prop => "Prop",
            Type => "Type",
            Assign => ":=",
            OpenParen => "(",
            CloseParen => ")",
            OpenBrace => "{",
            CloseBrace => "}",
            Ident => return None,
            Colon => ":",
            Pipe => "|",
            Arrow => "->",
            FatArrow => "=>",
            Fn => "fn",
            Forall => "forall",
            Comma => ",",
            Dot => ".",
        })
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{:?}`", self.kind)
    }
}