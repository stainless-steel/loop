use std::sync::mpsc;
use std::sync::{Arc, Mutex};

/// Process an iterator in parallel.
pub fn parallelize<Items, Map, Context, Item, Result>(
    items: Items,
    map: Map,
    context: Context,
    workers: Option<usize>,
) -> impl Iterator<Item = Result>
where
    Items: std::iter::Iterator<Item = Item> + Send + 'static,
    Map: Fn(Item, Context) -> Result + Copy + Send + 'static,
    Context: Clone + Send + 'static,
    Item: Send + 'static,
    Result: Send + 'static,
{
    let workers = crate::support::workers(workers);
    let (forward_sender, forward_receiver) = mpsc::sync_channel::<Item>(workers);
    let (backward_sender, backward_receiver) = mpsc::sync_channel::<Result>(workers);
    let forward_receiver = Arc::new(Mutex::new(forward_receiver));
    let mut _handlers = Vec::with_capacity(workers + 1);
    for _ in 0..workers {
        let forward_receiver = forward_receiver.clone();
        let backward_sender = backward_sender.clone();
        let context = context.clone();
        _handlers.push(std::thread::spawn(move || {
            while let Ok(Ok(item)) = forward_receiver.lock().map(|receiver| receiver.recv()) {
                if backward_sender.send(map(item, context.clone())).is_err() {
                    break;
                }
            }
        }));
    }
    _handlers.push(std::thread::spawn(move || {
        for item in items {
            if forward_sender.send(item).is_err() {
                break;
            }
        }
    }));
    backward_receiver.into_iter()
}

#[cfg(test)]
mod tests {
    #[test]
    fn parallelize() {
        let mut values = super::parallelize(0..10, map, 2, None).collect::<Vec<_>>();
        values.sort();
        assert_eq!(values, &[0, 2, 4, 6, 8, 10, 12, 14, 16, 18]);
    }

    fn map(item: i32, context: i64) -> usize {
        item as usize * context as usize
    }
}
