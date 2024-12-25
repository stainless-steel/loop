use std::sync::mpsc;
use std::sync::{Arc, Mutex};

/// Process an iterator in parallel.
pub fn parallelize<Inputs, Map, Context, Input, Output>(
    inputs: Inputs,
    map: Map,
    context: Context,
    workers: Option<usize>,
) -> impl Iterator<Item = Output>
where
    Inputs: std::iter::Iterator<Item = Input> + Send + 'static,
    Map: Fn(Input, Context) -> Output + Copy + Send + 'static,
    Context: Clone + Send + 'static,
    Input: Send + 'static,
    Output: Send + 'static,
{
    let workers = crate::support::workers(workers);
    let (forward_sender, forward_receiver) = mpsc::sync_channel::<Input>(workers);
    let (backward_sender, backward_receiver) = mpsc::sync_channel::<Output>(workers);
    let forward_receiver = Arc::new(Mutex::new(forward_receiver));
    let mut _handlers = Vec::with_capacity(workers + 1);
    for _ in 0..workers {
        let forward_receiver = forward_receiver.clone();
        let backward_sender = backward_sender.clone();
        let context = context.clone();
        _handlers.push(std::thread::spawn(move || {
            while let Ok(Ok(input)) = forward_receiver.lock().map(|receiver| receiver.recv()) {
                if backward_sender.send(map(input, context.clone())).is_err() {
                    break;
                }
            }
        }));
    }
    _handlers.push(std::thread::spawn(move || {
        for input in inputs {
            if forward_sender.send(input).is_err() {
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

    fn map(left: i32, right: i64) -> usize {
        left as usize * right as usize
    }
}
