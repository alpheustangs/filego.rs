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

This function splits file from a path to a directory directly. It will return the `file_size` and the `total_chunks` of the file.

```rust
use filego::{split, SplitOptions, SplitResult};

async fn example() {
    let options: SplitOptions = SplitOptions {
        in_file: "path/to/file".to_string(),
        out_dir: "path/to/dir".to_string(),
        chunk_size: 2 * 1024 * 1024,
    };

    let result: SplitResult = split(options).await.unwrap();
}
```

#### `check`

This function checks file integrity by verifying the the chunks specified in the `in_dir` with `file_size`, `total_chunks` parameters. It will return whether the check is successful with the `success` bool and the `error` struct from the check if any error occurs.

```rust
use filego::{check, CheckOptions, CheckResult};

async fn example() {
    let options: CheckOptions = CheckOptions {
        in_dir: "path/to/dir".to_string(),
        file_size: 0, // result from split function...
        total_chunks: 0, // result from split function...
    };

    let result: CheckResult = check(options).await.unwrap();
}
```

#### `merge`

This function merges the chunks from a directory to a specified path directly. Therefore, nothing will be returned as a result.

```rust
use filego::{merge, MergeOptions};

async fn example() {
    let options: MergeOptions = MergeOptions {
        in_dir: "path/to/dir".to_string(),
        out_file: "path/to/file".to_string(),
    };

    merge(options).await.unwrap();
}
```

## License

This project is MIT licensed, you can find the license file [here](./LICENSE).
