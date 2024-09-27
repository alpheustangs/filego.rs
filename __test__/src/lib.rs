#[cfg(test)]
mod tests {
    use std::{env, fs, path};

    use filego::{
        check, merge, split, CheckOptions, CheckResult, CheckResultErrorType,
        MergeOptions, SplitOptions, SplitResult,
    };
    use tokio::io as ioa;

    #[tokio::test]
    async fn test() {
        // declarations

        let root: path::PathBuf = env::current_dir().unwrap();

        let file_id: String = "test".to_string();
        let file_name: String = "test.txt".to_string();
        let chunk_size: usize = 2 * 1024 * 1024;

        let asset_path: path::PathBuf = root.join("assets").join(&file_name);
        let cache_dir: path::PathBuf =
            root.join(".media").join("cache").join(&file_id);
        let output_path: path::PathBuf =
            root.join(".media").join("output").join(file_id).join(file_name);

        // split

        let options: SplitOptions = SplitOptions {
            in_file: asset_path.to_string_lossy().to_string(),
            out_dir: cache_dir.to_string_lossy().to_string(),
            chunk_size,
        };

        let split_result: SplitResult = split(options).await.unwrap();

        assert!(
            fs::read_dir(cache_dir.as_path())
                .unwrap()
                .map(|res| res.map(|entry| entry.path()))
                .collect::<Result<Vec<_>, ioa::Error>>()
                .unwrap()
                .len()
                > 0
        );

        // check with missing error

        let options: CheckOptions = CheckOptions {
            in_dir: cache_dir.to_string_lossy().to_string(),
            file_size: split_result.file_size,
            total_chunks: split_result.total_chunks + 1,
        };

        let check_result: CheckResult = check(options).await.unwrap();

        assert_eq!(check_result.success, false);
        assert!(match check_result.error {
            | Some(ref e) => match e.error_type {
                | CheckResultErrorType::Missing => true,
                | _ => false,
            },
            | _ => false,
        });
        assert!(match check_result.error {
            | Some(ref e) => match e.error_type.to_string().as_str() {
                | "missing" => true,
                | _ => false,
            },
            | _ => false,
        });

        // check with size error

        let options: CheckOptions = CheckOptions {
            in_dir: cache_dir.to_string_lossy().to_string(),
            file_size: split_result.file_size + 1,
            total_chunks: split_result.total_chunks,
        };

        let check_result: CheckResult = check(options).await.unwrap();

        assert_eq!(check_result.success, false);
        assert!(match check_result.error {
            | Some(ref e) => match e.error_type {
                | CheckResultErrorType::Size => true,
                | _ => false,
            },
            | _ => false,
        });
        assert!(match check_result.error {
            | Some(ref e) => match e.error_type.to_string().as_str() {
                | "size" => true,
                | _ => false,
            },
            | _ => false,
        });

        // successful check

        let options: CheckOptions = CheckOptions {
            in_dir: cache_dir.to_string_lossy().to_string(),
            file_size: split_result.file_size,
            total_chunks: split_result.total_chunks,
        };

        let check_result: CheckResult = check(options).await.unwrap();

        assert_eq!(check_result.success, true);

        // merge

        let options: MergeOptions = MergeOptions {
            in_dir: cache_dir.to_string_lossy().to_string(),
            out_file: output_path.to_string_lossy().to_string(),
        };

        merge(options).await.unwrap();

        assert_eq!(output_path.exists(), true);
    }
}
