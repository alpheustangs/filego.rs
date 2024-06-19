use std::{fmt, path};

use tokio::{fs as fsa, io as ioa};

/// Options for the `check` function.
pub struct CheckOptions {
    /// Input directory to be checked in the `check` function.
    pub in_dir: String,
    /// Size of the original file,
    /// which can be found as an output of the `split` function.
    pub file_size: usize,
    /// Total number of chunks in the original file,
    /// which can be found as an output of the `split` function.
    pub total_chunks: usize,
}

/// Result error type of the `check` function.
pub enum CheckResultErrorType {
    /// Some of the chunks are missing to merge the file.
    Missing,
    /// The size of chunks do not match the `file_size` parameter.
    Size,
}

/// Result error of the `check` function.
pub struct CheckResultError {
    /// Type of error of the check.
    pub error_type: CheckResultErrorType,
    /// Error message of the check.
    pub message: String,
    /// Missing chunk(s) to merge the file.
    pub missing: Option<Vec<usize>>,
}

/// Result of the `check` function.
pub struct CheckResult {
    /// Successful / Failed check.
    pub success: bool,
    /// Error details of the check.
    pub error: Option<CheckResultError>,
}

/// This implements `to_string` function for `CheckResultErrorType`
/// to transfer the enum into string.
impl fmt::Display for CheckResultErrorType {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let s = match self {
            | CheckResultErrorType::Missing => "missing",
            | CheckResultErrorType::Size => "size",
        };
        write!(f, "{}", s)
    }
}

/// This function checks file integrity by verifying the the chunks specified
/// in the `in_dir` with `file_size`, `total_chunks` parameters.
/// It will return whether the check is successful
/// with the `success` bool and the `error` struct from the check if any error occurs.
///
/// ## Example
///
/// ```no_run
/// use filego::{check, CheckOptions, CheckResult};
///
/// async fn example() {
///     let options: CheckOptions = CheckOptions {
///         in_dir: "path/to/dir".to_string(),
///         file_size: 0, // result from split function...
///         total_chunks: 0, // result from split function...
///     };
///
///     let result: CheckResult = check(options).await.unwrap();
/// }
/// ```
pub async fn check(options: CheckOptions) -> ioa::Result<CheckResult> {
    let in_dir: &path::Path = path::Path::new(&options.in_dir);

    if !path::Path::new(in_dir).exists() {
        return Err(ioa::Error::new(
            ioa::ErrorKind::NotFound,
            "in_dir path not found",
        ));
    }

    if !path::Path::new(in_dir).is_dir() {
        return Err(ioa::Error::new(
            ioa::ErrorKind::InvalidInput,
            "in_dir is not a path to directory",
        ));
    }

    let mut actual_size: usize = 0;
    let mut missing: Vec<usize> = Vec::new();

    for i in 0..options.total_chunks {
        let target_file: path::PathBuf = in_dir.join(i.to_string());

        if !target_file.exists() || !target_file.is_file() {
            missing.push(i);
            continue;
        }

        actual_size += fsa::OpenOptions::new()
            .read(true)
            .open(&target_file)
            .await?
            .metadata()
            .await?
            .len() as usize;
    }

    if !missing.is_empty() {
        return Ok(CheckResult {
            success: false,
            error: Some(CheckResultError {
                error_type: CheckResultErrorType::Missing,
                message: "Missing chunk(s)".to_string(),
                missing: Some(missing),
            }),
        });
    }

    if actual_size != options.file_size {
        return Ok(CheckResult {
            success: false,
            error: Some(CheckResultError {
                error_type: CheckResultErrorType::Size,
                message:
                    "the size of chunks is not equal to file_size parameter"
                        .to_string(),
                missing: None,
            }),
        });
    }

    Ok(CheckResult { success: true, error: None })
}
