use iced_futures::subscription::Recipe;
use std::hash::Hasher;
use futures::stream::BoxStream;


pub struct Poll<H> {
    hash: H,
}

impl <H>Recipe<H, PollEvent> for Poll<H> where H: Hasher {
    type Output = PollEvent;

    fn hash(&self, state: &mut H) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.hash.hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, I>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        Box::pin(futures::stream::unfold(
            State::Ready(self.url),
            |state| async move {
                match state {
                    PollEvent::Ready() => {
                        match response {
                            Ok(response) => {
                            }
                            Err(_) => {
                            }
                        }
                    }
                    PollEvent::Pending {
                        mut response,
                        total,
                        downloaded,
                    } => match response.chunk().await {
                        Ok(Some(chunk)) => {
                            let downloaded = downloaded + chunk.len() as u64;

                            let percentage =
                                (downloaded as f32 / total as f32) * 100.0;

                            Some((
                                Progress::Advanced(percentage),
                                State::Downloading {
                                    response,
                                    total,
                                    downloaded,
                                },
                            ))
                        }
                        Ok(None) => Some((Progress::Finished, State::Finished)),
                        Err(_) => Some((Progress::Errored, State::Finished)),
                    },
                    PollEvent::Fulfilled => {
                        let _: () = iced::futures::future::pending().await;
                        None
                    }
                }
            },
        ))
    }
}

pub enum PollEvent {
    Ready,
    Pending,
    Fulfilled
}

use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

pub struct UpdateFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

impl Future for UpdateFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl UpdateFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        UpdateFuture { shared_state }
    }

    pub fn resolve() {
        let thread_shared_state = shared_state.clone();
        let mut shared_state = thread_shared_state.lock().unwrap();
        shared_state.completed = true;
        if let Some(waker) = shared_state.waker.take() {
            waker.wake()
        }
    }
}
