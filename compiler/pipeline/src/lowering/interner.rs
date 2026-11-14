use source::span::Span;
use surface::IdentId;

#[derive(Default)]
pub struct IdentInterner {
    identifiers: Vec<Span>,
}

impl IdentInterner {
    pub fn intern(&mut self, span: Span) -> IdentId {
        let id = IdentId(self.identifiers.len());
        self.identifiers.push(span);
        id
    }

    pub fn get(&self, id: IdentId) -> &Span {
        let IdentId(n) = id;
        &self.identifiers[n]
    }
}
