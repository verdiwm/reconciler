use std::future::Future;

use futures_util::{Stream, StreamExt};
use tokio::task::JoinSet;

pub struct EventHandler {
    streams: JoinSet<()>,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            streams: JoinSet::new(),
        }
    }

    pub fn add_handler<S, E, T, H, Fut>(&mut self, stream: S, handler: H)
    where
        S: Stream<Item = E> + Send + 'static,
        E: Into<T>,
        T: Send + 'static,
        H: FnMut(E) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send,
    {
        self.streams
            .spawn(async move { stream.for_each(handler).await });
    }
}
