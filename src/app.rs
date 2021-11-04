use super::event::{Event, Listener};
use super::graph::NodeGraph;
use iced::{executor, Application, Clipboard, Command, Element, Error, Settings, Subscription};
use std::cell::RefCell;
use std::collections::HashMap;
use iced_futures::subscription::Recipe;
use std::hash::{Hash, Hasher};

pub fn create_window(title: &'static str) -> Result<(), Error> {
    // DenoApplication::run(Settings::with_flags(Flags { title, nodes }))
    DenoApplication::run(Settings::with_flags(Flags { title }))
}

pub struct DenoApplication {
    title: &'static str,
    nodes: NodeGraph,
}

pub struct Flags {
    title: &'static str,
    // pub nodes: RefCell<NodeGraph>,
}

impl Application for DenoApplication {
    type Executor = executor::Default;
    type Message = Event;
    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut app = Self {
            title: flags.title,
            nodes: NodeGraph::new(),
        };
        // NODE_GRAPH.with(|nodes| {
        //     nodes.borrow_mut().update = Some(Box::new(|| {
        //         app.view();
        //     }));
        // });
        // app.nodes.borrow_mut().update = Some(Box::new(|| {
        //     app.view();
        // }));
        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from(self.title)
    }

    fn update(&mut self, _message: Event, _clipboard: &mut Clipboard) -> Command<Event> {
        Command::none()
    }

    fn view(&mut self) -> Element<Event> {
        NodeGraph::build_node(&mut self.nodes.body)
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }
}