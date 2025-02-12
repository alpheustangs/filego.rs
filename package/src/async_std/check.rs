use std::path::{Path, PathBuf};

use tokio::{fs, io};

use crate::check::{
    Check, CheckResult, CheckResultError, CheckResultErrorType,
};

/// Trait for running the check process.
pub trait CheckAsyncExt {
    /// Run the check process asynchronously.
    fn run_async(
        &self
    ) -> impl std::future::Future<Output = io::Result<CheckResult>> + Send;
}

impl CheckAsyncExt for Check {
    async fn run_async(&self) -> io::Result<CheckResult> {
        let in_dir: &Path = match self.in_dir {
            | Some(ref p) => {
                let p: &Path = p.as_ref();

                // if in_dir not exists
                if !p.exists() {
                    return Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        "in_dir path not found",
                    ));
                }

                // if in_dir not a directory
                if !p.is_dir() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "in_dir is not a directory",
                    ));
                }

                p
            },
            | None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "in_dir is not set",
                ))
            },
        };

        let file_size: usize = match self.file_size {
            | Some(s) => s,
            | None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "file_size is not set",
                ))
            },
        };

        let total_chunks: usize = match self.total_chunks {
            | Some(s) => s,
            | None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "total_chunks is not set",
                ))
            },
        };

        let mut actual_size: usize = 0;
        let mut missing: Vec<usize> = Vec::new();

        for i in 0..total_chunks {
            let target_file: PathBuf = in_dir.join(i.to_string());

            if !target_file.exists() || !target_file.is_file() {
                missing.push(i);
                continue;
            }

            actual_size += fs::OpenOptions::new()
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

        if actual_size != file_size {
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
