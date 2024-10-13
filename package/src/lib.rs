//! # FileGo
//!
//! A file splitting & merging solution.
//!
//! ## Quick Start
//!
//! Split file from a path to a directory with the `split` function.
//!
//! ```no_run
//! use std::path::PathBuf;
//!
//! use filego::split::{split, SplitOptions, SplitResult};
//!
//! async fn example() {
//!     let options: SplitOptions = SplitOptions {
//!         in_file: &PathBuf::from("path").join("to").join("file"),
//!         out_dir: &PathBuf::from("path").join("to").join("dir"),
//!         chunk_size: 2 * 1024 * 1024,
//!     };
//!
//!     let split_result: SplitResult = split(options).await.unwrap();
//! }
//! ```

mod functions;

/// Split module.
pub mod split {
    pub use crate::functions::split::{split, SplitOptions, SplitResult};
}

/// Check module.
pub mod check {
    pub use crate::functions::check::{
        check, CheckOptions, CheckResult, CheckResultError,
        CheckResultErrorType,
    };
}

/// Merge module.
pub mod merge {
    pub use crate::functions::merge::{merge, MergeOptions};
}
