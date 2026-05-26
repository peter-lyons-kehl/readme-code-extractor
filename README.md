# mce: Markdown Code Extractor

Rust macros to extract part(s) of `README.md` (or a similar Markdown file) and to use those parts in
tests, doctests or elsewhere.

## Why?

As per

- [stackoverflow#35080160](https://stackoverflow.com/questions/35080160/github-include-md-files-in-readme-md),
- [github/markup#346](https://github.com/github/markup/issues/346) and
- [github/markup#172](https://github.com/github/markup/issues/172)

GitHub does not support/allow `README.md` (or `README.rst`) to include other file(s).

Rust supports the other way: including `README.md` (or any file) as a part of rustdoc, usually done
in `src/lib.rs` with:
```rust
#![doc = include_str!("../README.md")]`
```

However, it's not possible to pick up just specific parts of `README.md` (or another file). That
would be beneficial, for example when re-using specific code sections in both `README.md` and
negative/positive doctests, or when re-using specific code sections with specific cargo features or
configuration.

There are several crates that generate `README.md` based on other file(s). But that needs CI setup
or manual steps. Those needs to be manually checked, which is error prone and time-consuming.

Instead, `mce` extracts the code section(s) with no side effects. It's stateless.

## MSRV

MSRV (minimum supported Rust version), as indicated with `rust-version` in
[`Cargo.toml`](Cargo.toml), is 1.88.0. However, it's tested only with Rust 1.89.0+. You will get
incorrect (false positive) `dead_code` warnings, unless you use Rust 1.89.0+. (See
[rust-lang/rust#142208](https://github.com/rust-lang/rust/pull/142208).)

## Related issues

Please give thumbs up (and contribute, if you can) to

- [SergioBenitez/proc-macro2-diagnostics#13](https://github.com/SergioBenitez/proc-macro2-diagnostics/issues/13)
  defect: Error message and details missing, when macro fails to generate main() on STABLE
- [dtolnay/no-panic#28](https://github.com/dtolnay/no-panic/issues/28) `no-panic` currently cannot be
  used in doctests
- [dtolnay/no-panic#78](https://github.com/dtolnay/no-panic/issues/78) Not working (false negative)
  in integration tests

## Side fruit

- [dtolnay/proc-macro2#536](https://github.com/dtolnay/proc-macro2/pull/536) `Span::file` and
  `Span::local_file` suggest to use Rust 1.88 when using stable
