use std::error::Error;

use futures_util::Stream;
use tokio::sync::{broadcast, mpsc};
use tokio_stream::StreamExt;

/// Event priority
///
/// # Note
///
/// This is not actually used at the moment
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Critical,
    High,
    Normal,
    Low,
}

pub enum SourceError<E> {
    // TODO: more variants
    Source(E),
}

#[derive(Debug, Clone)]
pub struct Event<T> {
    source_id: String,
    priority: Priority,
    inner: T,
}

pub struct Manager<T, E> {
    tx: mpsc::Sender<Event<T>>,
    error_tx: mpsc::Sender<SourceError<E>>,
    error_rx: Option<mpsc::Receiver<SourceError<E>>>,
}

impl<T: Send + 'static, E: Error + Send + 'static> Manager<T, E> {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(100);
        // let (broadcast, _) = broadcast::channel(broadcast_size);
        let (error_tx, error_rx) = mpsc::channel(100);

        Self {
            tx,
            // rx: Some(rx),
            // broadcast,
            error_tx,
            error_rx: Some(error_rx),
        }
    }

    pub fn add_source<S>(&self, source_id: String, stream: S)
    where
        S: Stream<Item = Result<Event<T>, SourceError<E>>> + Send + Unpin + 'static,
    {
        let tx = self.tx.clone();
        let error_tx = self.error_tx.clone();

        tokio::spawn(async move {
            let mut stream = stream;

            while let Some(result) = stream.next().await {
                match result {
                    Ok(event) => {
                        if tx.send(event).await.is_err() {
                            break;
                        }
                    }
                    Err(error) => {
                        let _ = error_tx.send(error).await;
                    }
                }
            }
        });
    }
}
