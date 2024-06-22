# FileGo

A solution for splitting, checking and merging files.

## Installation

To install FileGo, run the following command:

```bash
cargo add filego
```

## Quick Start

Split file from a path to a directory with the `split` function.

```rust
use filego::{split, SplitOptions, SplitResult};

async fn example() {
    let options: SplitOptions = SplitOptions {
        in_file: "path/to/file".to_string(),
        out_dir: "path/to/dir".to_string(),
        chunk_size: 2 * 1024 * 1024,
    };

    let split_result: SplitResult = split(options).await.unwrap();
}
```

## License

This project is MIT licensed, you can find the license file [here](./LICENSE).
