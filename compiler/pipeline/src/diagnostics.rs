use syntax::node::Node;

pub struct Diagnostic {}

impl Diagnostic {
    pub fn unexpected_node(_node: &Node) -> Self {
        Self {}
    }
}
