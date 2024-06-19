use std::path;

use tokio::{
    fs as fsa,
    io::{self as ioa, AsyncReadExt, AsyncWriteExt},
};

/// Options for the `split` function.
pub struct SplitOptions {
    /// Input file to be splitted in the `split` function.
    pub in_file: String,
    /// Output directory after splitted in the `split` function.
    pub out_dir: String,
    /// Size of each chunk in byte to be splitted.
    pub chunk_size: usize,
}

/// Result of the `split` function.
pub struct SplitResult {
    /// Size of the original file.
    pub file_size: usize,
    /// The total number of chunks splitted from the original file.
    pub total_chunks: usize,
}

/// This function splits file from a path to a directory directly.
/// It will return the `file_size` and the `total_chunks` of the file.
///
/// ## Example
///
/// ```no_run
/// use filego::{split, SplitOptions, SplitResult};
///
/// async fn example() {
///     let options: SplitOptions = SplitOptions {
///         in_file: "path/to/file".to_string(),
///         out_dir: "path/to/dir".to_string(),
///         chunk_size: 2 * 1024 * 1024,
///     };
///
///     let result: SplitResult = split(options).await.unwrap();
/// }
/// ```
pub async fn split(options: SplitOptions) -> ioa::Result<SplitResult> {
    // declarations
    let in_file: &path::Path = path::Path::new(&options.in_file);
    let out_dir: &path::Path = path::Path::new(&options.out_dir);
    let chunk_size: usize = options.chunk_size;

    // if inpath not exists
    if !in_file.exists() {
        return Err(ioa::Error::new(
            ioa::ErrorKind::NotFound,
            "in_file path not found",
        ));
    }

    // if inpath not file
    if !in_file.is_file() {
        return Err(ioa::Error::new(
            ioa::ErrorKind::InvalidInput,
            "in_file is not a path to file",
        ));
    }

    let input: fsa::File =
        fsa::OpenOptions::new().read(true).open(in_file).await?;

    let file_size: usize = input.metadata().await?.len() as usize;

    let buffer_capacity: usize = chunk_size.min(10 * 1024 * 1024);

    let mut reader: ioa::BufReader<fsa::File> =
        ioa::BufReader::with_capacity(buffer_capacity, input);

    // if outdir not exists
    if !out_dir.exists() {
        fsa::create_dir_all(out_dir).await?;
    }

    let mut buffer: Vec<u8> = vec![0; chunk_size];

    let mut total_chunks: usize = 0;

    let mut current: usize = 0;

    loop {
        let read: usize = reader.read(&mut buffer[current..]).await?;

        if read == 0 {
            if current > 0 {
                // write the remaining data
                let output_path: path::PathBuf =
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
            let output_path: path::PathBuf =
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
