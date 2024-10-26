use std::{
    fs,
    path::{Path, PathBuf},
};

use tokio::{
    fs as fsa,
    io::{self as ioa, AsyncReadExt, AsyncWriteExt},
};

use crate::config::BUFFER_CAPACITY_MAX;

/// Process to merge chunks from a directory to a path.
///
/// ## Example
///
/// ```no_run
/// use std::path::PathBuf;
///
/// use filego::merge::Merge;
///
/// async fn example() {
///     let result: bool = Merge::new()
///         .in_dir(PathBuf::from("path").join("to").join("dir"))
///         .out_file(PathBuf::from("path").join("to").join("file"))
///         .run()
///         .await
///         .unwrap();
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Merge {
    in_dir: Option<PathBuf>,
    out_file: Option<PathBuf>,
    cap_max: usize,
}

impl Merge {
    /// Create a new merge process.
    pub fn new() -> Self {
        Self { in_dir: None, out_file: None, cap_max: BUFFER_CAPACITY_MAX }
    }

    /// Set the input directory.
    pub fn in_dir<InDir: AsRef<Path>>(
        mut self,
        in_dir: InDir,
    ) -> Self {
        self.in_dir = Some(in_dir.as_ref().to_path_buf());
        self
    }

    /// Set the output file.
    pub fn out_file<OutFile: AsRef<Path>>(
        mut self,
        out_file: OutFile,
    ) -> Self {
        self.out_file = Some(out_file.as_ref().to_path_buf());
        self
    }

    /// Set the maximum size of the buffer capacity.
    ///
    /// By default, the buffer capacity is based on the size of the inputs in
    /// the input directory. The buffer capacity is limited and will not
    /// exceed [`BUFFER_CAPACITY_MAX`]. The default value is recommended
    /// unless a large size file will be processed through the split process.
    pub fn max_buffer_capacity(
        mut self,
        capacity: usize,
    ) -> Self {
        self.cap_max = capacity;
        self
    }

    /// Run the merge process.
    pub async fn run(self) -> ioa::Result<bool> {
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

        let out_file: &Path = match self.out_file {
            | Some(ref p) => p.as_ref(),
            | None => {
                return Err(ioa::Error::new(
                    ioa::ErrorKind::InvalidInput,
                    "out_file is not set",
                ))
            },
        };

        // check file size for buffer capacity
        let input_size: usize = if let Some(file) = fs::read_dir(in_dir)?
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_file())
            .map(|entry| entry.path())
            .next()
        {
            fsa::metadata(file).await?.len() as usize
        } else {
            return Err(ioa::Error::new(
                ioa::ErrorKind::NotFound,
                "No files found in in_dir",
            ));
        };

        let buffer_capacity: usize = input_size.min(self.cap_max);

        // delete outpath target if exists
        if out_file.exists() {
            if out_file.is_dir() {
                fsa::remove_dir_all(&out_file).await?;
            } else {
                fsa::remove_file(&out_file).await?;
            }
        }

        // create outpath
        if let Some(parent) = out_file.parent() {
            fsa::create_dir_all(parent).await?;
        }

        let output: fsa::File = fsa::OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(out_file)
            .await?;

        // writer
        let mut writer: ioa::BufWriter<fsa::File> =
            ioa::BufWriter::with_capacity(buffer_capacity, output);

        // get inputs
        let mut entries: Vec<PathBuf> = fs::read_dir(in_dir)?
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_file())
            .map(|entry| entry.path())
            .collect();

        entries.sort_by_key(|entry| {
            entry
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .parse::<usize>()
                .unwrap()
        });

        // merge
        for entry in entries {
            let input: fsa::File =
                fsa::OpenOptions::new().read(true).open(&entry).await?;

            let mut reader: ioa::BufReader<fsa::File> =
                ioa::BufReader::with_capacity(buffer_capacity, input);

            let mut buffer: Vec<u8> = vec![0; buffer_capacity];

            loop {
                let read: usize = reader.read(&mut buffer).await?;

                if read == 0 {
                    break;
                }

                writer.write_all(&buffer[..read]).await?;
            }
        }

        writer.flush().await?;

        Ok(true)
    }
}

impl Default for Merge {
    fn default() -> Self {
        Self::new()
    }
}
