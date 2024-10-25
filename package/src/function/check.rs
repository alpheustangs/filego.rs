use std::path::{Path, PathBuf};

use tokio::{fs as fsa, io as ioa};

/// Result error type of the `check` function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// Process to check the file integrity.
///
/// The function will return [`CheckResult`] (that may come
/// with `success:false`) when the checking process runs successfully.
/// Otherwise, it will return Error.
///
/// ## Example
///
/// ```no_run
/// use std::path::PathBuf;
///
/// use filego::check::{Check, CheckResult};
///
/// async fn example() {
///     let result: CheckResult = Check::new()
///         .in_dir(PathBuf::from("path").join("to").join("dir"))
///         .file_size(0) // result from split function...
///         .total_chunks(0) // result from split function...
///         .run()
///         .await
///         .unwrap();
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Check<InDir: AsRef<Path>> {
    in_dir: Option<InDir>,
    file_size: usize,
    total_chunks: usize,
}

impl<InDir: AsRef<Path>> Check<InDir> {
    /// Create a new check process.
    pub fn new() -> Self {
        Self { in_dir: None, file_size: 0, total_chunks: 0 }
    }

    /// Set the input directory.
    pub fn in_dir(
        mut self,
        in_dir: InDir,
    ) -> Self {
        self.in_dir = Some(in_dir);
        self
    }

    /// Set the size of the original file.
    pub fn file_size(
        mut self,
        file_size: usize,
    ) -> Self {
        self.file_size = file_size;
        self
    }

    /// Set the total number of chunks splitted from the original file.
    pub fn total_chunks(
        mut self,
        total_chunks: usize,
    ) -> Self {
        self.total_chunks = total_chunks;
        self
    }

    /// Run the check process.
    pub async fn run(self) -> ioa::Result<CheckResult> {
        let in_dir: &Path = match self.in_dir {
            | Some(ref p) => {
                let p: &Path = p.as_ref();

                // if in_dir not exists
                if !p.exists() {
                    return Err(ioa::Error::new(
                        ioa::ErrorKind::NotFound,
                        "in_dir path not found",
                    ));
                }

                // if in_dir not a directory
                if !p.is_dir() {
                    return Err(ioa::Error::new(
                        ioa::ErrorKind::InvalidInput,
                        "in_dir is not a directory",
                    ));
                }

                p
            },
            | None => {
                return Err(ioa::Error::new(
                    ioa::ErrorKind::InvalidInput,
                    "in_dir is not set",
                ))
            },
        };

        let mut actual_size: usize = 0;
        let mut missing: Vec<usize> = Vec::new();

        for i in 0..self.total_chunks {
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

        if actual_size != self.file_size {
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
}

impl<P: AsRef<Path>> Default for Check<P> {
    fn default() -> Self {
        Self::new()
    }
}
