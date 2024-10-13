use std::path::PathBuf;

use tokio::{fs as fsa, io as ioa};

/// Options for the `check` function.
#[derive(Debug, Clone)]
pub struct CheckOptions<'a> {
    /// Input directory to be checked in the `check` function.
    pub in_dir: &'a PathBuf,
    /// Size of the original file,
    /// which can be found as an output of the `split` function.
    pub file_size: usize,
    /// Total number of chunks in the original file,
    /// which can be found as an output of the `split` function.
    pub total_chunks: usize,
}

/// Result error type of the `check` function.
#[derive(Debug, Clone)]
pub enum CheckResultErrorType {
    /// Some of the chunks are missing to merge the file.
    Missing,
    /// The size of chunks do not match the `file_size` parameter.
    Size,
}

impl CheckResultErrorType {
    /// Get the code of the error type as `&str`.
    pub fn as_code(&self) -> &str {
        match self {
            | Self::Missing => "missing",
            | Self::Size => "size",
        }
    }

    /// Get the code of the error type as `String`.
    pub fn to_code(&self) -> String {
        self.as_code().to_string()
    }
}

/// Result error of the `check` function.
#[derive(Debug, Clone)]
pub struct CheckResultError {
    /// Type of error of the check.
    pub error_type: CheckResultErrorType,
    /// Error message of the check.
    pub message: String,
    /// Missing chunk(s) to merge the file.
    pub missing: Option<Vec<usize>>,
}

/// Result of the `check` function.
#[derive(Debug, Clone)]
pub struct CheckResult {
    /// Successful / Failed check.
    pub success: bool,
    /// Error details of the check.
    pub error: Option<CheckResultError>,
}

/// This function checks file integrity by verifying the the chunks specified
/// in the `in_dir` with `file_size`, `total_chunks` parameters.
/// It will return whether the check is successful
/// with the `success` bool and the `error` struct from the check if any error occurs.
///
/// ## Example
///
/// ```no_run
/// use std::path::PathBuf;
///
/// use filego::check::{check, CheckOptions, CheckResult};
///
/// async fn example() {
///     let options: CheckOptions = CheckOptions {
///         in_dir: &PathBuf::from("path").join("to").join("dir"),
///         file_size: 0, // result from split function...
///         total_chunks: 0, // result from split function...
///     };
///
///     let result: CheckResult = check(options).await.unwrap();
/// }
/// ```
pub async fn check(options: CheckOptions<'_>) -> ioa::Result<CheckResult> {
    let in_dir: &PathBuf = options.in_dir;

    if !in_dir.exists() {
        return Err(ioa::Error::new(
            ioa::ErrorKind::NotFound,
            "in_dir path not found",
        ));
    }

    if !in_dir.is_dir() {
        return Err(ioa::Error::new(
            ioa::ErrorKind::InvalidInput,
            "in_dir is not a path to directory",
        ));
    }

    let mut actual_size: usize = 0;
    let mut missing: Vec<usize> = Vec::new();

    for i in 0..options.total_chunks {
        let target_file: PathBuf = in_dir.join(i.to_string());

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
