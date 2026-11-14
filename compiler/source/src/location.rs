use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Hash, Default)]
pub struct Location {
    pub position: usize,
    pub line: usize,
    pub column: usize,
}
