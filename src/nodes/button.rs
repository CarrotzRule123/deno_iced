use super::super::state::State;
use iced::button;

#[derive(Clone)]
pub struct ButtonNode {
    pub rid: u64,
    pub text: String,
    pub state: button::State,
}

impl ButtonNode {
    pub fn new(rid: u64) -> Self {
        Self {
            rid,
            text: String::from(""),
            state: button::State::new()
        }
    }

    pub fn set_state(&mut self, state: State) {
        match state {
            State::Text(text) => self.text = text,
            _ => panic!()
        };
    } 
}