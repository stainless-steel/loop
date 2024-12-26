use std::sync::Arc;

use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::ReceiverStream;

/// Process an iterator in parallel.
pub fn parallelize<Items, Map, Context, Item, Future, Output>(
    items: Items,
    map: Map,
    context: Context,
    workers: Option<usize>,
) -> impl futures::stream::Stream<Item = Output>
where
    Items: Iterator<Item = Item> + Send + 'static,
    Map: Fn(Item, Context) -> Future + Copy + Send + 'static,
    Context: Clone + Send + 'static,
    Item: Copy + Send + 'static,
    Future: std::future::Future<Output = Output> + Send,
    Output: Send + 'static,
{
    let workers = crate::support::workers(workers);
    let (item_sender, item_receiver) = mpsc::channel::<Item>(workers);
    let (output_sender, output_receiver) = mpsc::channel::<Output>(workers);
    let item_receiver = Arc::new(Mutex::new(item_receiver));
    let mut _handlers = Vec::with_capacity(workers + 1);
    for _ in 0..workers {
        let item_receiver = item_receiver.clone();
        let output_sender = output_sender.clone();
        let context = context.clone();
        _handlers.push(tokio::task::spawn(async move {
            while let Some(item) = item_receiver.lock().await.recv().await {
                if output_sender
                    .send(map(item, context.clone()).await)
                    .await
                    .is_err()
                {
                    break;
                }
            }
        }));
    }
    _handlers.push(tokio::task::spawn(async move {
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
        let mut values = super::parallelize(0..10, map, 2, None)
            .collect::<Vec<_>>()
            .await;
        values.sort();
        assert_eq!(values, &[0, 2, 4, 6, 8, 10, 12, 14, 16, 18]);
    }

    async fn map(item: i32, right: i64) -> usize {
        item as usize * right as usize
    }
}
