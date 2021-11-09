use iced::{Button, Column, Element, Row, Text};
use rand::{Rng, rngs};
use std::cell::RefCell;
use std::collections::HashMap;

use super::event::{Event, Listener};
use super::nodes::{DivNode, Node, NodeProto, Nodes};
use super::state::Align;
use super::app::Message;

pub struct NodeGraph {
    pub update: Option<Box<dyn FnMut()>>,
    pub body: Node,
    nodes: HashMap<u64, NodeProto>,
    rand: rngs::ThreadRng,
}

impl NodeGraph {
    pub fn new() -> Self {
        let mut rand = rand::thread_rng();

        let proto = NodeProto {
            rid: rand.gen(),
            path: vec![],
            // node: Nodes::Div
        };
        let body = Node::Div(DivNode::new());
        let mut nodes = HashMap::new();
        nodes.insert(proto.rid, proto);
        Self { nodes, body, rand, update: None }
    }

    pub fn create_proto(&mut self) -> u64 {
        let proto = NodeProto {
            rid: self.rand.gen(),
            path: vec![],
        };
        self.nodes.insert(proto.rid, proto.clone());
        proto.rid
    }

    pub fn traverse<'a>(node: &'a mut Node, path: &Vec<usize>, i: usize) -> &'a mut Node {
        if path.len() < i {
            let next = match node {
                Node::Div(div) => &mut div.children[path[i]],
                _ => panic!(),
            };
            return NodeGraph::traverse(next, path, i + 1)
        };
        node
    }

    pub fn lookup(&mut self, rid: u64) -> &mut Node {
        let proto = self.nodes.get(&rid).unwrap();
        NodeGraph::traverse(&mut self.body, &proto.path, 0)
    }

    pub fn add_child(&mut self, parent: u64, child: Nodes) -> u64 {
        let proto = self.create_proto();
        let node1 = self.lookup(parent);
        let node2 = Node::from(proto, child);
        match node1 {
            Node::Div(div) => div.children.push(node2),
            _ => panic!(),
        };
        if let Some(update) = &mut self.update {
            (**update)();
        };
        proto
    }

    pub fn build_node<'a>(node: &'a mut Node) -> Element<'a, Message> {
        match node {
            Node::Div(div) => match div.align {
                Align::Horizontal => {
                    let mut row = Row::new();
                    for child in &mut div.children {
                        row = row.push(NodeGraph::build_node(child));
                    };
                    row.into()
                },
                Align::Vertical => Column::new().into()
            },
            Node::Button(button) => {
                Button::new(&mut button.state, Text::new(button.text.clone())).into()
            },
            Node::Text(text) => {
                Text::new(text.text.clone()).into()
            },
            _ => panic!()
        }
    }
}
