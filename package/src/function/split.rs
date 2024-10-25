use std::path::{Path, PathBuf};

use tokio::{
    fs as fsa,
    io::{self as ioa, AsyncReadExt, AsyncWriteExt},
};

use crate::config::{BUFFER_CAPACITY_MAX, CHUNK_SIZE_DEFAULT};

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
///         .in_file(PathBuf::from("path").join("to").join("file"))
///         .out_dir(PathBuf::from("path").join("to").join("dir"))
///         .run()
///         .await
///         .unwrap();
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Split<P: AsRef<Path>> {
    in_file: Option<P>,
    out_dir: Option<P>,
    chunk_size: usize,
    cap_max: usize,
}

/// Result of the `split` function.
#[derive(Debug, Clone)]
pub struct SplitResult {
    /// Size of the original file.
    pub file_size: usize,
    /// The total number of chunks splitted from the original file.
    pub total_chunks: usize,
}

impl<P: AsRef<Path>> Split<P> {
    /// Create a new split process.
    pub fn new() -> Self {
        Self {
            in_file: None,
            out_dir: None,
            chunk_size: CHUNK_SIZE_DEFAULT,
            cap_max: BUFFER_CAPACITY_MAX,
        }
    }

    /// Set the input file.
    pub fn in_file(
        mut self,
        in_file: P,
    ) -> Self {
        self.in_file = Some(in_file);
        self
    }

    /// Set the output directory.
    pub fn out_dir(
        mut self,
        out_dir: P,
    ) -> Self {
        self.out_dir = Some(out_dir);
        self
    }

    /// Set the maximum size of each chunk.
    ///
    /// By default, the chunk size follows the [`CHUNK_SIZE_DEFAULT`].
    pub fn chunk_size(
        mut self,
        chunk_size: usize,
    ) -> Self {
        self.chunk_size = chunk_size;
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

impl<P: AsRef<Path>> Default for Split<P> {
    fn default() -> Self {
        Self::new()
    }
}
