# FileGo

A file splitting & merging solution.

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

This project is MIT licensed, you can find the license file 
[here](https://github.com/alpheustangs/filego.rs/blob/main/LICENSE).
