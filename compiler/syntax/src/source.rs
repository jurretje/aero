use std::fmt::Debug;

use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Hash, Default)]
pub struct Location {
    pub pos: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, PartialEq, Eq, Serialize, Hash)]
pub struct Span {
    pub start: Location,
    pub end: Location,
}

impl Span {
    pub fn text<'a>(&self, input: &'a str) -> &'a str {
        &input[self.start.pos..self.end.pos]
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}|{}..{}|{}]",
            self.start.line, self.start.column, self.end.line, self.end.column
        )
    }
}

#[derive(Clone)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(node: T, span: Span) -> Self {
        Self { node, span }
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
