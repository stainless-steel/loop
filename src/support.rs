pub fn workers(value: Option<usize>) -> usize {
    value.unwrap_or_else(|| {
        std::thread::available_parallelism()
            .map(|value| value.get())
            .unwrap_or(1)
    })
}
