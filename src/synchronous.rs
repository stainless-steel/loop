use std::sync::{mpsc, Arc, Mutex};

/// Process an iterator in parallel.
pub fn parallelize<Items, Map, Context, Item, Output>(
    items: Items,
    map: Map,
    context: Context,
    workers: Option<usize>,
) -> impl Iterator<Item = Output>
where
    Items: Iterator<Item = Item> + Send + 'static,
    Map: Fn(Item, Context) -> Output + Copy + Send + 'static,
    Context: Clone + Send + 'static,
    Item: Send + 'static,
    Output: Send + 'static,
{
    let workers = crate::support::workers(workers);
    let (item_sender, item_receiver) = mpsc::sync_channel::<Item>(workers);
    let (output_sender, output_receiver) = mpsc::sync_channel::<Output>(workers);
    let item_receiver = Arc::new(Mutex::new(item_receiver));
    let mut _handlers = Vec::with_capacity(workers + 1);
    for _ in 0..workers {
        let item_receiver = item_receiver.clone();
        let output_sender = output_sender.clone();
        let context = context.clone();
        _handlers.push(std::thread::spawn(move || {
            while let Ok(Ok(item)) = item_receiver.lock().map(|receiver| receiver.recv()) {
                if output_sender.send(map(item, context.clone())).is_err() {
                    break;
                }
            }
        }));
    }
    _handlers.push(std::thread::spawn(move || {
        for item in items {
            if item_sender.send(item).is_err() {
                break;
            }
        }
    }));
    output_receiver.into_iter()
}

#[cfg(test)]
mod tests {
    #[test]
    fn parallelize() {
        let mut values = super::parallelize(0..10, map, 2, None).collect::<Vec<_>>();
        values.sort();
        assert_eq!(values, &[0, 2, 4, 6, 8, 10, 12, 14, 16, 18]);
    }

    fn map(left: i32, right: i64) -> usize {
        left as usize * right as usize
    }
}
