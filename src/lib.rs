//! Processing iterators in parallel.
//!
//! # Example
//!
//! ```
//! # #[cfg(not(feature = "asynchronous"))]
//! fn main() {
//!     let multiply = |value| 2 * value;
//!     let _ = r#loop::parallelize(0..10, multiply, None).collect::<Vec<_>>();
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
//!     let multiply = |value| async move { 2 * value };
//!     let _ = r#loop::parallelize(0..10, multiply, None).collect::<Vec<_>>().await;
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
