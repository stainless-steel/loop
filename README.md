# Loop [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The package allows for processing iterators in parallel.

# Example

```rust
let map = |item: &_, context| std::io::Result::Ok(*item * context);
let (items, results): (Vec<_>, Vec<_>) = r#loop::parallelize(0..10, map, 2, None).unzip();
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[build-img]: https://github.com/stainless-steel/loop/workflows/build/badge.svg
[build-url]: https://github.com/stainless-steel/loop/actions/workflows/build.yml
[documentation-img]: https://docs.rs/loop/badge.svg
[documentation-url]: https://docs.rs/loop
[package-img]: https://img.shields.io/crates/v/loop.svg
[package-url]: https://crates.io/crates/loop
