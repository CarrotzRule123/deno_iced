use iced::{button};

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Click
}

#[derive(Debug, Clone, Copy)]
pub enum State {
    Text
}

pub enum Listener {
    Button(button::State)
}