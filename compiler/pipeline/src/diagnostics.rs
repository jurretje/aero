use elaborator::ElabError;
use syntax::node::Node;

#[derive(Debug)]
pub enum Diagnostic {
    UnexpectedNode(Node),
    ElabError(ElabError)
}

impl Diagnostic {
    pub fn unexpected_node(node: &Node) -> Self {
        Self::UnexpectedNode(node.clone())
    }
}
