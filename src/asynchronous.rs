//! Asynchronous implementation.

use std::sync::Arc;

use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::ReceiverStream;

/// Process an iterator in parallel.
pub fn parallelize<Items, Item, Map, Future, Output>(
    items: Items,
    mut map: Map,
    workers: Option<usize>,
) -> impl futures::stream::Stream<Item = Output>
where
    Items: IntoIterator<Item = Item> + Send + 'static,
    <Items as IntoIterator>::IntoIter: Send,
    Item: Send + 'static,
    Map: FnMut(Item) -> Future + Copy + Send + 'static,
    Future: std::future::Future<Output = Output> + Send,
    Output: Send + 'static,
{
    let workers = crate::support::workers(workers);
    let (item_sender, item_receiver) = mpsc::channel::<Item>(workers);
    let (output_sender, output_receiver) = mpsc::channel::<Output>(workers);
    let item_receiver = Arc::new(Mutex::new(item_receiver));
    for _ in 0..workers {
        let item_receiver = item_receiver.clone();
        let output_sender = output_sender.clone();
        std::mem::drop(tokio::task::spawn(async move {
            while let Some(item) = {
                let mut receiver = item_receiver.lock().await;
                receiver.recv().await
            } {
                if output_sender.send(map(item).await).await.is_err() {
                    break;
                }
            }
        }));
    }
    std::mem::drop(tokio::task::spawn(async move {
        for item in items {
            if item_sender.send(item).await.is_err() {
                break;
            }
        }
    }));
    ReceiverStream::new(output_receiver)
}

#[cfg(test)]
mod tests {
    use futures::stream::StreamExt;

    #[tokio::test]
    async fn parallelize() {
        let mut values = super::parallelize(0..10, double, None)
            .collect::<Vec<_>>()
            .await;
        values.sort();
        assert_eq!(values, &[0, 2, 4, 6, 8, 10, 12, 14, 16, 18]);
    }

    async fn double(value: usize) -> usize {
        2 * value
    }
}
