//! Processing iterators in parallel.
//!
//! # Example
//!
//! ```
//! # #[cfg(not(feature = "asynchronous"))]
//! fn main() {
//!     let map = |item, context| item * context;
//!     let _ = r#loop::parallelize(0..10, map, 2, None).collect::<Vec<_>>();
//! }
//! # #[cfg(feature = "asynchronous")]
//! # fn main() {}
//!```
//!
//!```
//! # #[cfg(feature = "asynchronous")]
//! #[tokio::main]
//! async fn main() {
//!     use futures::stream::StreamExt;
//!
//!     let map = |item, context| async move { item * context };
//!     let _ = r#loop::parallelize(0..10, map, 2, None).collect::<Vec<_>>().await;
//! }
//! # #[cfg(not(feature = "asynchronous"))]
//! # fn main() {}
//! ```

#[cfg(feature = "asynchronous")]
#[path = "asynchronous.rs"]
mod implementation;

#[cfg(not(feature = "asynchronous"))]
#[path = "synchronous.rs"]
mod implementation;

mod support;

pub use implementation::parallelize;
