# Changelog

## unpublished (only more important decision points listed)

### May 3, 2026

Renamed from readme-code-extractor to mce (Markdown Code Extractor).

### May 1, 2026

Error handling:

- `public::ext` module (extension traits) for
  - errors implementing `Debug`, `ToString` or `Into<String>`
  - error indicators
    - containers holding those errors: `Result` and `Option`
    - `bool`
- `public::assert` module with more intentional function names

### April 30, 2026

Error handling: `proc_macro2_diagnostics` crate +`DeepDiagnostic`.

### April 27, 2026

- `all_by_file` macro

### April 25, 2026

- stopped using `trybuild` crate in [negative_tests_runner/`](negative_tests_runner/). See
  [`prudent-rs/mce -> negative_tests_runner/src/lib.rs commit
  #ff8935f`](https://github.com/prudent-rs/mce/blob/ff8935ff314133ddc432c32d0ec89c41f4dd0dd0/negative_tests_runner/src/lib.rs).
