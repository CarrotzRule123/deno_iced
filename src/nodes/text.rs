use super::super::state::State;

#[derive(Clone)]
pub struct TextNode {
    pub text: String,
}

impl TextNode {
    pub fn new() -> Self {
        Self {
            text: String::from(""),
        }
    }

    pub fn set_state(&mut self, state: State) {
        match state {
            State::Text(text) => self.text = text,
            _ => panic!()
        };
    } 
}