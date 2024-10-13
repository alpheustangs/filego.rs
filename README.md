# FileGo

A file splitting & merging solution.

## Installation

To install this package, run the following command:

```bash
cargo add filego
```

## Quick Start

Split file from a path to a directory with the `split` function.

```rust
use std::path::PathBuf;

use filego::split::{split, SplitOptions, SplitResult};

async fn example() {
    let options: SplitOptions = SplitOptions {
        in_file: &PathBuf::from("path").join("to").join("file"),
        out_dir: &PathBuf::from("path").join("to").join("dir"),
        chunk_size: 2 * 1024 * 1024,
    };

    let split_result: SplitResult = split(options).await.unwrap();
}
```

## License

This project is MIT licensed, you can find the license file [here](./LICENSE).
