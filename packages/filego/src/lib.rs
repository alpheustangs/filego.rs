mod functions;

pub use functions::check::{
    check, CheckOptions, CheckResult, CheckResultError, CheckResultErrorType,
};
pub use functions::merge::{merge, MergeOptions};
pub use functions::split::{split, SplitOptions, SplitResult};
