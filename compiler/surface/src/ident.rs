use std::fmt::Debug;

use syntax::source::Span;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Ident {
    name: String,
    span: Span,
}

impl Ident {
    pub fn new(name: String, span: Span) -> Self {
        Self { name, span }
    }

    pub fn literal(&self) -> &str {
        &self.name
    }
}

impl Debug for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}
