## 0.4.0 (2024-10-26)

### Breaking Changes

- Rework in `split` module
- Rework in `check` module
- Rework in `merge` module

Please refer to docs for the new usage.

### What's New

- Add `config` module

### What's Changed

- Update dependencies
- Update documentation

## 0.3.0 (2024-10-13)

### Breaking Changes

- Move `split` related stuffs into `split` module
- Move `check` related stuffs into `check` module
- Move `merge` related stuffs into `merge` module
- Changes in accepted value type of `in_file` in `SplitOptions`:
    - `String` => `&PathBuf`
- Changes in accepted value type of `out_dir` in `SplitOptions`:
    - `String` => `&PathBuf`
- Changes in accepted value type of `in_dir` in `CheckOptions`:
    - `String` => `&PathBuf`
- Remove `std::fmt::Display` impl from `CheckResultErrorType`
- Changes in accepted value type of `in_dir` in `MergeOptions`:
    - `String` => `&PathBuf`
- Changes in accepted value type of `out_file` in `MergeOptions`:
    - `String` => `&PathBuf`

### What's New

- Add different derives for different structs
- Add `as_code` function for `CheckResultErrorType`
- Add `to_code` function for `CheckResultErrorType`

## 0.2.3 (2024-08-04)

### What's Changed

- Update description
- Update dependencies

## 0.2.2 (2024-06-22)

### What's Changed

- Update docs

## 0.2.1 (2024-06-22)

### What's Changed

- Update `Cargo.toml`

## 0.2.0 (2024-06-19)

### What's New

- Add inline documentation for structs and functions

### What's Changed

- Update dependencies

## 0.1.0 (2024-05-22)

First release
