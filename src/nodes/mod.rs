mod button;
mod div;
mod text;

pub use button::ButtonNode;
pub use div::DivNode;
pub use text::TextNode;
use num_derive::FromPrimitive;
use super::state::State;


#[derive(FromPrimitive, Debug, Clone, Copy)]
pub enum Nodes {
    Button = 0,
    Text,
    Div,
}

#[derive(Clone)]
pub enum Node {
    Button(ButtonNode),
    Text(TextNode),
    Div(DivNode),
}

#[derive(Clone)]
pub struct NodeProto {
    pub path: Vec<usize>,
    pub rid: u64,
    // pub node: Nodes
}

impl Node {
    pub fn from(rid: u64, node: Nodes) -> Node {
        match node {
            Nodes::Button => Node::Button(ButtonNode::new(rid)),
            Nodes::Div => Node::Div(DivNode::new()),
            Nodes::Text => Node::Text(TextNode::new())
        }
    }

    pub fn set_state(&mut self, state: State) {
        match self {
            Node::Button(node) => node.set_state(state),
            Node::Div(node) => node.set_state(state),
            Node::Text(node) => node.set_state(state),
            // _ => panic!()
        };
    } 
}