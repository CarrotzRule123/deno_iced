use super::event::{Event, Listener};
use super::graph::NodeGraph;
use super::NODE_GRAPH;
use iced::{executor, Application, Clipboard, Command, Element, Error, Settings};
use std::cell::RefCell;
use std::collections::HashMap;

pub fn create_window(title: &'static str) -> Result<(), Error> {
    // DenoApplication::run(Settings::with_flags(Flags { title, nodes }))
    NODE_GRAPH.with(|nodes| {
        DenoApplication::run(Settings::with_flags(Flags { title }))
    })
}

pub struct DenoApplication {
    title: &'static str,
    listeners: HashMap<u64, Listener>,
    // pub nodes: RefCell<NodeGraph>,
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
        let mut listeners = HashMap::new();
        let mut app = Self {
            title: flags.title,
            listeners
            // nodes: flags.nodes,
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
        NODE_GRAPH.with(move |nodes| {
            let nodes_ref = nodes.borrow_mut();
            let nodes_copy = nodes_ref.body.clone();
            NodeGraph::build_node(&mut self.listeners, nodes_copy)
        })
    }
}
