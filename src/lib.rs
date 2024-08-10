use futures_util::{Stream, StreamExt};
use pin_project_lite::pin_project;
use tokio::{sync::mpsc, task::JoinSet};
use tokio_stream::wrappers::ReceiverStream;
use tokio_util::sync::PollSender;

pub mod handler;

pin_project! {
    pub struct EventListener<T> {
        sender: PollSender<T>,
        #[pin]
        receiver: ReceiverStream<T>,
        streams: JoinSet<()>,
    }
}

impl<T: Send + 'static> EventListener<T> {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel::<T>(100);

        Self {
            sender: PollSender::new(sender),
            receiver: ReceiverStream::new(receiver),
            streams: JoinSet::new(),
        }
    }

    pub fn add_listener<S, E>(&mut self, stream: S)
    where
        S: Stream<Item = E> + Send + 'static,
        E: Into<T>,
    {
        let sender = self.sender.clone();

        self.streams.spawn({
            async move {
                let _ = stream.map(|event| Ok(event.into())).forward(sender).await;
            }
        });
    }

    // TODO: add util functions for failable listeners
}

impl<T> Stream for EventListener<T> {
    type Item = T;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.project().receiver.poll_next(cx)
    }
}
