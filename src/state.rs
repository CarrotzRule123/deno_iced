use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::str;

#[derive(FromPrimitive)]
pub enum States {
    Text = 0,
    Align = 1
}

#[derive(Debug, Clone)]
pub enum State {
    Text(String),
    Align(Align)
}

#[derive(Debug, Clone, FromPrimitive)]
pub enum Align {
    Vertical = 0,
    Horizontal
}

impl State {
    pub fn from(state: States, buf: &[u8]) -> Self {
        match state {
            States::Text => {
                let string = str::from_utf8(buf).unwrap();
                State::Text(String::from(string))
            }
            States::Align => {
                State::Align(FromPrimitive::from_u8(buf[0]).unwrap())
            }
        }
    }
}
