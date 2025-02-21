//! # FileGo
//!
//! A file splitting & merging solution.
//!
//! ## Quick Start
//!
//! Split file from a path to a directory with `Split` struct.
//!
//! ```no_run
//! use std::path::PathBuf;
//!
//! use filego::split::{Split, SplitResult};
//!
//! fn example() {
//!     let result: SplitResult = Split::new()
//!         .in_file(PathBuf::from("path").join("to").join("file"))
//!         .out_dir(PathBuf::from("path").join("to").join("dir"))
//!         .run()
//!         .unwrap();
//! }
//! ```
//!
//! Async version also available with the `async-std` and `tokio` features:
//!
//! ```rust
//! // This is a `async-std` example
//!
//! use async_std::path::PathBuf;
//!
//! use filego::split::{
//!     Split,
//!     SplitResult,
//!     async_std::SplitAsyncExt as _,
//! };
//!
//! async fn example() {
//!     let result: SplitResult = Split::new()
//!         .in_file(PathBuf::from("path").join("to").join("file"))
//!         .out_dir(PathBuf::from("path").join("to").join("dir"))
//!         .run_async()
//!         .await
//!         .unwrap();
//! }
//! ```
//!
//! ```rust
//! // This is a `tokio` example
//!
//! use std::path::PathBuf;
//!
//! use filego::split::{
//!     Split,
//!     SplitResult,
//!     tokio::SplitAsyncExt as _,
//! };
//!
//! async fn example() {
//!     let result: SplitResult = Split::new()
//!         .in_file(PathBuf::from("path").join("to").join("file"))
//!         .out_dir(PathBuf::from("path").join("to").join("dir"))
//!         .run_async()
//!         .await
//!         .unwrap();
//! }
//! ```

/// The default chunk size in bytes.
pub const CHUNK_SIZE_DEFAULT: usize = 2 * 1024 * 1024;

/// The default maximum size of the buffer capacity in bytes.
pub const BUFFER_CAPACITY_MAX_DEFAULT: usize = 10 * 1024 * 1024;

/// Split module.
pub mod split;

/// Check module.
pub mod check;

/// Merge module.
pub mod merge;

/// Functions implemented with `async-std`.
#[cfg(feature = "async-std")]
pub(crate) mod async_std;

/// Functions implemented with `tokio`.
#[cfg(feature = "tokio")]
pub(crate) mod tokio;
