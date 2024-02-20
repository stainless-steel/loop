//! Processing iterators in parallel.
//!
//! # Example
//!
//! ```
//! let map = |item: &_, context| std::io::Result::Ok(*item * context);
//! let (items, results): (Vec<_>, Vec<_>) = r#loop::parallelize(0..10, map, 2, None).unzip();
//! ```

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

/// Process an iterator in parallel.
pub fn parallelize<Iterator, Item, Map, Context, Value, Error>(
    iterator: Iterator,
    map: Map,
    context: Context,
    workers: Option<usize>,
) -> impl DoubleEndedIterator<Item = (Item, Result<Value, Error>)>
where
    Iterator: std::iter::Iterator<Item = Item>,
    Map: Fn(&Item, Context) -> Result<Value, Error> + Copy + Send + 'static,
    Item: Send + 'static,
    Context: Clone + Send + 'static,
    Value: Send + 'static,
    Error: Send + 'static,
{
    let (forward_sender, forward_receiver) = mpsc::channel::<Item>();
    let (backward_sender, backward_receiver) = mpsc::channel::<(Item, Result<Value, Error>)>();
    let forward_receiver = Arc::new(Mutex::new(forward_receiver));

    let workers = workers.unwrap_or_else(|| {
        std::thread::available_parallelism()
            .map(|value| value.get())
            .unwrap_or(1)
    });
    let _ = (0..workers)
        .map(|_| {
            let forward_receiver = forward_receiver.clone();
            let backward_sender = backward_sender.clone();
            let context = context.clone();
            thread::spawn(move || loop {
                let entry = match forward_receiver.lock().unwrap().recv() {
                    Ok(entry) => entry,
                    Err(_) => break,
                };
                let result = map(&entry, context.clone());
                backward_sender.send((entry, result)).unwrap();
            })
        })
        .collect::<Vec<_>>();
    let mut count = 0;
    for entry in iterator {
        forward_sender.send(entry).unwrap();
        count += 1;
    }
    (0..count).map(move |_| backward_receiver.recv().unwrap())
}

#[cfg(test)]
mod tests {
    macro_rules! ok(($result:expr) => ($result.unwrap()));

    #[test]
    fn parallelize() {
        let values = super::parallelize(0..10, map, 2, None)
            .map(|(_, result)| ok!(result))
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        assert_eq!(values, &[0, 2, 4, 6, 8, 10, 12, 14, 16, 18]);
    }

    fn map(item: &i32, context: i64) -> std::io::Result<usize> {
        Ok(*item as usize * context as usize)
    }
}
