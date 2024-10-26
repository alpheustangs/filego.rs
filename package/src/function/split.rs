use std::path::{Path, PathBuf};

use tokio::{
    fs as fsa,
    io::{self as ioa, AsyncReadExt, AsyncWriteExt},
};

use crate::config::{BUFFER_CAPACITY_MAX_DEFAULT, CHUNK_SIZE_DEFAULT};

/// Process to split file from a path to a directory.
///
/// ## Example
///
/// ```no_run
/// use std::path::PathBuf;
///
/// use filego::split::{Split, SplitResult};
///
/// async fn example() {
///     let result: SplitResult = Split::new()
///         .in_file("/path/to/file")
///         .out_dir(PathBuf::from("path").join("to").join("dir"))
///         .run()
///         .await
///         .unwrap();
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Split {
    in_file: Option<PathBuf>,
    out_dir: Option<PathBuf>,
    chunk_size: usize,
    cap_max: usize,
}

/// Result of the split process.
#[derive(Debug, Clone)]
pub struct SplitResult {
    /// Size of the original file.
    pub file_size: usize,
    /// The total number of chunks splitted from the original file.
    pub total_chunks: usize,
}

impl Split {
    /// Create a new split process.
    pub fn new() -> Self {
        Self {
            in_file: None,
            out_dir: None,
            chunk_size: CHUNK_SIZE_DEFAULT,
            cap_max: BUFFER_CAPACITY_MAX_DEFAULT,
        }
    }

    /// Set the input file.
    pub fn in_file<InFile: AsRef<Path>>(
        mut self,
        path: InFile,
    ) -> Self {
        self.in_file = Some(path.as_ref().to_path_buf());
        self
    }

    /// Set the output directory.
    pub fn out_dir<OutDir: AsRef<Path>>(
        mut self,
        path: OutDir,
    ) -> Self {
        self.out_dir = Some(path.as_ref().to_path_buf());
        self
    }

    /// Set the maximum size of each chunk.
    ///
    /// By default, the chunk size follows the [`CHUNK_SIZE_DEFAULT`].
    pub fn chunk_size(
        mut self,
        size: usize,
    ) -> Self {
        self.chunk_size = size;
        self
    }

    /// Set the maximum size of the buffer capacity.
    ///
    /// By default, the buffer capacity is based on the `chunk_size`.
    /// The buffer capacity is limited and will not exceed
    /// [`BUFFER_CAPACITY_MAX`]. The default value is recommended unless
    /// a large size file will be processed through the split process.
    pub fn max_buffer_capacity(
        mut self,
        capacity: usize,
    ) -> Self {
        self.cap_max = capacity;
        self
    }

    /// Run the split process.
    pub async fn run(self) -> ioa::Result<SplitResult> {
        let in_file: &Path = match self.in_file {
            | Some(ref p) => {
                let p: &Path = p.as_ref();

                // if in_file not exists
                if !p.exists() {
                    return Err(ioa::Error::new(
                        ioa::ErrorKind::NotFound,
                        "in_file path not found",
                    ));
                }

                // if in_file not a file
                if !p.is_file() {
                    return Err(ioa::Error::new(
                        ioa::ErrorKind::InvalidInput,
                        "in_file is not a path to file",
                    ));
                }

                p
            },
            | None => {
                return Err(ioa::Error::new(
                    ioa::ErrorKind::InvalidInput,
                    "in_file is not set",
                ))
            },
        };

        let out_dir: &Path = match self.out_dir {
            | Some(ref p) => {
                let p: &Path = p.as_ref();

                // if out_dir not exists
                if !p.exists() {
                    fsa::create_dir_all(&p).await?;
                } else {
                    // if out_dir not a directory
                    if p.is_file() {
                        return Err(ioa::Error::new(
                            ioa::ErrorKind::InvalidInput,
                            "out_dir is not a directory",
                        ));
                    }
                }

                p
            },
            | None => {
                return Err(ioa::Error::new(
                    ioa::ErrorKind::InvalidInput,
                    "out_dir is not set",
                ))
            },
        };

        let chunk_size: usize = self.chunk_size;

        let buffer_capacity: usize = chunk_size.min(self.cap_max);

        let input: fsa::File =
            fsa::OpenOptions::new().read(true).open(in_file).await?;

        let file_size: usize = input.metadata().await?.len() as usize;

        let mut reader: ioa::BufReader<fsa::File> =
            ioa::BufReader::with_capacity(buffer_capacity, input);

        let mut buffer: Vec<u8> = vec![0; chunk_size];

        let mut total_chunks: usize = 0;

        let mut current: usize = 0;

        loop {
            let read: usize = reader.read(&mut buffer[current..]).await?;

            if read == 0 {
                if current > 0 {
                    // write the remaining data
                    let output_path: PathBuf =
                        out_dir.join(total_chunks.to_string());

                    let output: fsa::File = fsa::OpenOptions::new()
                        .create(true)
                        .truncate(true)
                        .write(true)
                        .open(output_path)
                        .await?;

                    let mut writer: ioa::BufWriter<fsa::File> =
                        ioa::BufWriter::with_capacity(buffer_capacity, output);

                    writer.write_all(&buffer[..current]).await?;

                    writer.flush().await?;

                    total_chunks += 1;
                }

                break;
            }

            current += read;

            if current >= chunk_size {
                // write chunk
                let output_path: PathBuf =
                    out_dir.join(total_chunks.to_string());

                let output: fsa::File = fsa::OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .open(output_path)
                    .await?;

                let mut writer: ioa::BufWriter<fsa::File> =
                    ioa::BufWriter::with_capacity(buffer_capacity, output);

                writer.write_all(&buffer[..chunk_size]).await?;

                writer.flush().await?;

                total_chunks += 1;

                // move remaining data to the start of the buffer
                buffer.copy_within(chunk_size..current, 0);
                current -= chunk_size;
            }
        }

        Ok(SplitResult { file_size, total_chunks })
    }
}

impl Default for Split {
    fn default() -> Self {
        Self::new()
    }
}
