use std::sync::Arc;

use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio_stream::wrappers::ReceiverStream;

/// Process an iterator in parallel.
pub fn parallelize<Inputs, Map, Context, Input, Future, Output>(
    inputs: Inputs,
    map: Map,
    context: Context,
    workers: Option<usize>,
) -> impl futures::stream::Stream<Item = Output>
where
    Inputs: std::iter::Iterator<Item = Input> + Send + 'static,
    Map: Fn(Input, Context) -> Future + Copy + Send + 'static,
    Context: Clone + Send + 'static,
    Input: Copy + Send + 'static,
    Future: std::future::Future<Output = Output> + Send,
    Output: Send + 'static,
{
    let workers = crate::support::workers(workers);
    let (forward_sender, forward_receiver) = mpsc::channel::<Input>(workers);
    let (backward_sender, backward_receiver) = mpsc::channel::<Output>(workers);
    let forward_receiver = Arc::new(Mutex::new(forward_receiver));
    let mut _handlers = Vec::with_capacity(workers + 1);
    for _ in 0..workers {
        let forward_receiver = forward_receiver.clone();
        let backward_sender = backward_sender.clone();
        let context = context.clone();
        _handlers.push(tokio::task::spawn(async move {
            while let Some(input) = forward_receiver.lock().await.recv().await {
                if backward_sender
                    .send(map(input, context.clone()).await)
                    .await
                    .is_err()
                {
                    break;
                }
            }
        }));
    }
    _handlers.push(tokio::task::spawn(async move {
        for input in inputs {
            if forward_sender.send(input).await.is_err() {
                break;
            }
        }
    }));
    ReceiverStream::new(backward_receiver)
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
