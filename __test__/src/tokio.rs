#[cfg(test)]
mod tests {
    use std::{env, path::PathBuf};

    use tokio::fs::{self, ReadDir};

    use filego::{
        check::{
            Check, CheckResult, CheckResultErrorType, tokio::CheckAsyncExt as _,
        },
        merge::{Merge, tokio::MergeAsyncExt as _},
        split::{Split, SplitResult, tokio::SplitAsyncExt as _},
    };

    async fn setup(
        cache_name: &str
    ) -> (PathBuf, PathBuf, PathBuf, SplitResult) {
        let root: PathBuf = env::current_dir().unwrap();
        let file_name: &str = "test.png";
        let chunk_size: usize = 1024 * 1024;

        let asset_path: PathBuf = root.join("assets").join(file_name);
        let cache_dir: PathBuf =
            root.join(".media").join("cache").join("tokio").join(cache_name);

        // split file
        let split_result: SplitResult = Split::new()
            .in_file(&asset_path)
            .out_dir(&cache_dir)
            .chunk_size(chunk_size)
            .run_async()
            .await
            .unwrap();

        (
            root.clone(),
            cache_dir,
            root.join(".media")
                .join("output")
                .join("tokio")
                .join(cache_name)
                .join(file_name),
            split_result,
        )
    }

    #[tokio::test]
    async fn test_split_file_creates_chunks() {
        let (_, cache_dir, _, _) = setup("split_file_creates_chunks").await;

        let mut read_dir: ReadDir = fs::read_dir(&cache_dir).await.unwrap();

        let mut chunk_count: usize = 0;

        while let Ok(Some(_)) = read_dir.next_entry().await {
            chunk_count += 1;
        }

        assert!(chunk_count > 0, "No chunks were created.");
    }

    #[tokio::test]
    async fn test_check_with_missing_chunks() {
        let (_, cache_dir, _, split_result) =
            setup("check_with_missing_chunks").await;

        let check_result: CheckResult = Check::new()
            .in_dir(&cache_dir)
            .file_size(split_result.file_size)
            .total_chunks(split_result.total_chunks + 1)
            .run_async()
            .await
            .unwrap();

        assert!(
            !check_result.success,
            "Check should fail due to missing chunks."
        );
        if let Some(e) = check_result.error {
            assert_eq!(e.error_type, CheckResultErrorType::Missing);
            assert_eq!(e.error_type.as_code(), "missing");
        }
    }

    #[tokio::test]
    async fn test_check_with_size_error() {
        let (_, cache_dir, _, split_result) =
            setup("check_with_size_error").await;

        let check_result: CheckResult = Check::new()
            .in_dir(&cache_dir)
            .file_size(split_result.file_size + 1)
            .total_chunks(split_result.total_chunks)
            .run_async()
            .await
            .unwrap();

        assert!(
            !check_result.success,
            "Check should fail due to size mismatch."
        );
        if let Some(e) = check_result.error {
            assert_eq!(e.error_type, CheckResultErrorType::Size);
            assert_eq!(e.error_type.as_code(), "size");
        }
    }

    #[tokio::test]
    async fn test_successful_check() {
        let (_, cache_dir, _, split_result) = setup("successful_check").await;

        let check_result: CheckResult = Check::new()
            .in_dir(&cache_dir)
            .file_size(split_result.file_size)
            .total_chunks(split_result.total_chunks)
            .run_async()
            .await
            .unwrap();

        assert!(
            check_result.success == true,
            "Check should succeed with no errors."
        );
    }

    #[tokio::test]
    async fn test_merge_creates_output_file() {
        let (_, cache_dir, output_path, _) =
            setup("merge_creates_output_file").await;

        Merge::new()
            .in_dir(&cache_dir)
            .out_file(&output_path)
            .run_async()
            .await
            .unwrap();

        assert!(
            output_path.exists(),
            "Output file should be created after merging."
        );
    }

    #[tokio::test]
    async fn test_merge_on_empty_cache_dir() {
        let root: PathBuf = env::current_dir().unwrap();
        let empty_cache_dir: PathBuf =
            root.join(".media").join("cache").join("tokio").join("empty_test");

        fs::create_dir_all(&empty_cache_dir).await.unwrap();

        let output_path: PathBuf = root
            .join(".media")
            .join("output")
            .join("tokio")
            .join("empty_test")
            .join("output.txt");

        assert!(
            Merge::new()
                .in_dir(&empty_cache_dir)
                .out_file(&output_path)
                .run_async()
                .await
                .is_err(),
            "Merge should fail with an empty cache directory."
        );
    }
}
