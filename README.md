# FileGo

A file splitting & merging solution.

## Installation

To install this package, run the following command:

```bash
cargo add filego
```

## Quick Start

Split file from a path to a directory with `Split` struct.

```rust
use std::path::PathBuf;

use filego::split::{Split, SplitResult};

async fn example() {
    let result: SplitResult = Split::new()
        .in_file("/path/to/file")
        .out_dir(PathBuf::from("path").join("to").join("dir"))
        .run()
        .await
        .unwrap();
}
```

## License

This project is licensed under the terms of the MIT license.
