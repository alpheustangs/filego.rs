use std::{fmt, path};

use tokio::{fs as fsa, io as ioa};

pub struct CheckOptions {
    /// path to input directory
    pub in_dir: String,
    /// input file size in byte
    ///
    /// `file_size` should be equal to `file_size` in `SplitResult`
    pub file_size: usize,
    /// number of chunks
    ///
    /// `total_chunks` should be equal to `total_chunks` in `SplitResult`
    pub total_chunks: usize,
}

pub enum CheckResultErrorType {
    /// error type: Missing chunk(s)
    Missing,
    /// error type: Unmatched file size
    Size,
}

pub struct CheckResultError {
    /// error type
    pub error_type: CheckResultErrorType,
    /// error message
    pub message: String,
    /// missing chunk(s) if any
    pub missing: Option<Vec<usize>>,
}

pub struct CheckResult {
    /// success or not
    pub success: bool,
    /// error on fail
    pub error: Option<CheckResultError>,
}

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
