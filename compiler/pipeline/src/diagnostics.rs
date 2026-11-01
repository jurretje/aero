use syntax::node::Node;

#[derive(Debug)]
pub struct Diagnostic {
    unexpected_node: Node,
}

impl Diagnostic {
    pub fn unexpected_node(node: &Node) -> Self {
        Self {
            unexpected_node: node.clone()
        }
    }
}
