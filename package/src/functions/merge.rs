use std::{fs, path::PathBuf};

use tokio::{
    fs as fsa,
    io::{self as ioa, AsyncReadExt, AsyncWriteExt},
};

/// Options for the `merge` function.
#[derive(Debug, Clone)]
pub struct MergeOptions<'a> {
    /// Input directory to be merged in the `merge` function.
    pub in_dir: &'a PathBuf,
    /// Output file after merging in the `merge` function.
    pub out_file: &'a PathBuf,
}

/// This function merges the chunks from a directory to a specified path directly.
/// Therefore, nothing will be returned as a result.
///
/// ## Example
///
/// ```no_run
/// use std::path::PathBuf;
///
/// use filego::merge::{merge, MergeOptions};
///
/// async fn example() {
///     let options: MergeOptions = MergeOptions {
///         in_dir: &PathBuf::from("path").join("to").join("dir"),
///         out_file: &PathBuf::from("path").join("to").join("file"),
///     };
///
///     merge(options).await.unwrap();
/// }
/// ```
pub async fn merge(options: MergeOptions<'_>) -> ioa::Result<()> {
    // declarations
    let in_dir: &PathBuf = options.in_dir;
    let out_file: &PathBuf = options.out_file;

    // if indir not exists
    if !in_dir.exists() {
        return Err(ioa::Error::new(
            ioa::ErrorKind::NotFound,
            "in_dir path not found",
        ));
    }

    // if indir not dir
    if !in_dir.is_dir() {
        return Err(ioa::Error::new(
            ioa::ErrorKind::InvalidInput,
            "in_dir is not a path to direcotry",
        ));
    }

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

    let buffer_capacity: usize = input_size.min(10 * 1024 * 1024);

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
        entry.file_name().unwrap().to_str().unwrap().parse::<usize>().unwrap()
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

    Ok(())
}
