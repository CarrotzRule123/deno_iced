use iced::{executor, Application, Clipboard, Command, Element, Error, Settings, Subscription};
use iced_futures::subscription::Recipe;
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use super::event::{Event, Listener};
use super::graph::NodeGraph;
use super::update::{Update, UpdateFuture, UpdateSub};
use super::RESOURCES;

pub fn create_window(id: u64, title: &'static str) -> Result<(), Error> {
    DenoApplication::run(Settings::with_flags(Flags { title, id }))
    // DenoApplication::run(Settings::with_flags(Flags { title }))
}

pub struct DenoApplication {
    title: &'static str,
    nodes: NodeGraph,
}

pub struct Flags {
    title: &'static str,
    id: u64
    // pub nodes: RefCell<NodeGraph>,
}

impl Application for DenoApplication {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let app = Self {
            title: flags.title,
            nodes: NodeGraph::new(flags.id),
        };
        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from(self.title)
    }

    fn update(&mut self, _message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        NodeGraph::build_node(&mut self.nodes.body)
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::from_recipe(UpdateSub {}).map(Message::Update)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Update(Update),
    Event(Event),
}
