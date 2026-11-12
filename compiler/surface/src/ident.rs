#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdentId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualifiedIdent {
    pub segments: Vec<IdentId>,
}
