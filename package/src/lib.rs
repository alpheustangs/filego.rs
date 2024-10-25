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
//! async fn example() {
//!     let result: SplitResult = Split::new()
//!         .in_file("/path/to/file")
//!         .out_dir(PathBuf::from("path").join("to").join("dir"))
//!         .run()
//!         .await
//!         .unwrap();
//! }
//! ```

mod function;

/// Config module.
pub mod config;

/// Split module.
pub mod split {
    pub use crate::function::split::*;
}

/// Check module.
pub mod check {
    pub use crate::function::check::*;
}

/// Merge module.
pub mod merge {
    pub use crate::function::merge::*;
}
