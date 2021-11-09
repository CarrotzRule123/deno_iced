use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::hash::{Hash, Hasher};
use futures::stream::BoxStream;
use iced_futures::subscription::Recipe;

use super::RESOURCES;

pub struct UpdateFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    update: Update,
    waker: Option<Waker>,
}

impl Future for &UpdateFuture {
    type Output = Update;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Update> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(Update::AddChild)
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl UpdateFuture {
    pub fn new() -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            update: Update::AddChild,
            waker: None,
        }));
        Self { shared_state }
    }

    pub fn update(&self, update: Update) {
        let thread_shared_state = self.shared_state.clone();
        let mut shared_state = thread_shared_state.lock().unwrap();
        shared_state.completed = true;
        shared_state.update = update;
        if let Some(waker) = shared_state.waker.take() {
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
    AddChild,
    SetState,
}