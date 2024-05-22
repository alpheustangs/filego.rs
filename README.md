# FileGo

Solution for splitting, checking and merging files.

## Install

To install FileGo, run the following command:

```bash
cargo add filego
```

## Usage

Usage of different functions in FileGo:

#### `split`

Split files from a file path to a directory directly. It will return the `file_size` and the `total_chunks` of the file.

```rust
use filego::{split, SplitOptions, SplitResult};

let options: SplitOptions = SplitOptions {
    in_file: "path/to/file".to_string(),
    out_dir: "path/to/dir".to_string(),
    chunk_size: 2 * 1024 * 1024,
};

let result: SplitResult = split(options).await.unwrap();
```

#### `check`

Check file integrity by verifying the the chunks specified in the `in_dir` with `file_size`, `total_chunks` parameters. It will return whether the check is successful with the `success` bool and the `error` struct from the check if any error occurs.

```rust
use filego::{check, CheckOptions, CheckResult};

let options: CheckOptions = CheckOptions {
    in_dir: "path/to/dir".to_string(),
    file_size: 123, // file size from split result
    total_chunks: 123, // total chunks from splut result
};

let result: CheckResult = check(options).await.unwrap();
```

#### `merge`

Merge the chunks from the directory to a specified path directly. Nothing will be returned as a result.

```rust
use filego::{merge, MergeOptions};

let options: MergeOptions = MergeOptions {
    in_dir: "path/to/dir".to_string(),
    out_file: "path/to/file".to_string(),
};

merge(options).await.unwrap();
```

## License

This project is MIT licensed, you can find the license file [here](./LICENSE).
