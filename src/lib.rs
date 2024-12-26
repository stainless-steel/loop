//! Processing iterators in parallel.
//!
//! # Examples
//!
//! Synchronously:
//!
//! ```
//! # #[cfg(not(feature = "asynchronous"))]
//! fn main() {
//!     let double = |value| 2 * value;
//!     let _ = r#loop::parallelize(0..10, double).collect::<Vec<_>>();
//! }
//! # #[cfg(feature = "asynchronous")]
//! # fn main() {}
//!```
//!
//! Asynchronously:
//!
//!```
//! # #[cfg(feature = "asynchronous")]
//! #[tokio::main]
//! async fn main() {
//!     use futures::stream::StreamExt;
//!
//!     let double = |value| async move { 2 * value };
//!     let _ = r#loop::parallelize(0..10, double).collect::<Vec<_>>().await;
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
