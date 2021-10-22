use super::super::state::{State, Align};
use super::Node;

#[derive(Clone)]
pub struct DivNode {
    pub children: Vec<Node>,
    pub align: Align
}

impl DivNode {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            align: Align::Horizontal
        }
    }

    pub fn set_state(&mut self, state: State) {
        match state {
            State::Align(align) => self.align = align,
            _ => panic!()
        };
    } 
}