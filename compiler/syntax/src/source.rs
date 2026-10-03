use std::fmt::Debug;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Location {
    pub pos: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, PartialEq, Eq)]
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