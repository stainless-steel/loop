//! Processing iterators in parallel.
//!
//! # Examples
//!
//! Synchronously:
//!
//! ```
//! fn main() {
//!     use r#loop::parallelize;
//!
//!     let double = |value| 2 * value;
//!     let _ = parallelize(0..10, double, None).collect::<Vec<_>>();
//! }
//!```
//!
//! Asynchronously:
//!
//!```
//! # #[cfg(feature = "asynchronous")]
//! #[tokio::main]
//! async fn main() {
//!     use futures::stream::StreamExt;
//!     use r#loop::asynchronous::parallelize;
//!
//!     let double = |value| async move { 2 * value };
//!     let _ = parallelize(0..10, double, None).collect::<Vec<_>>().await;
//! }
//! # #[cfg(not(feature = "asynchronous"))]
//! # fn main() {}
//! ```

#[cfg(feature = "asynchronous")]
pub mod asynchronous;
pub mod synchronous;

mod support;

pub use synchronous::parallelize;
