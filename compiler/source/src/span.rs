use std::fmt::Debug;

use serde::Serialize;

use crate::location::Location;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Hash)]
pub struct Span {
    pub start: Location,
    pub end: Location,
}

impl Span {
    pub fn text<'s>(&self, s: &'s str) -> &'s str {
        &s[self.start.position..self.end.position]
    }

    pub fn combine(first: Self, second: Self) -> Self {
        Self {
            start: first.start,
            end: second.end,
        }
    }
}

#[derive(Clone)]
pub struct Spanned<T> {
    pub span: Span,
    pub node: T,
}

impl<T> Spanned<T> {
    pub fn new(span: Span, node: T) -> Self {
        Self { span, node }
    }

    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Spanned<U> {
        Spanned {
            node: f(self.node),
            span: self.span,
        }
    }
}

impl<T: Debug> Debug for Spanned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.node)
        } else {
            write!(f, "{:?}", self.node)
        }
    }
}
