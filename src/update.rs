use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::hash::{Hash, Hasher};
use futures::stream::BoxStream;
use iced_futures::subscription::Recipe;

use super::RESOURCES;
use super::nodes::Nodes;

pub struct UpdateFuture {
    state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    update: Update,
    waker: Option<Waker>,
}

impl Future for &UpdateFuture {
    type Output = Update;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Update> {
        let mut state = self.state.lock().unwrap();
        if state.completed {
            Poll::Ready(state.update)
        } else {
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl UpdateFuture {
    pub fn new() -> Self {
        let state = Arc::new(Mutex::new(SharedState {
            completed: false,
            update: Update::None,
            waker: None,
        }));
        Self { state }
    }

    pub fn update(&self, update: Update) {
        let thread_state = self.state.clone();
        let mut state = thread_state.lock().unwrap();
        state.completed = true;
        state.update = update;
        if let Some(waker) = state.waker.take() {
            waker.wake()
        }
    }
}

pub struct UpdateSub {}

impl<H, E> Recipe<H, E> for UpdateSub where H: Hasher {
    type Output = Update;

    fn hash(&self, state: &mut H) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);
        // self.id.hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: BoxStream<'static, E>,
    ) -> BoxStream<'static, Self::Output> {
        Box::pin(futures::stream::unfold(
            true,
            move |_| async move {
                let future = &RESOURCES.update;
                Some((future.await, true))
            },
        ))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Update {
    AddChild(Nodes, u64, u64),
    SetState,
    None
}