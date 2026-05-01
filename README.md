# readme-code-extractor

Rust macros to extract part(s) of README.md (or a similar file) and to use them in
tests/doctests/elsewhere.

## MSRV
MSRV (minimum supported Rust version), as indicated with `rust-version` in
[`Cargo.toml`](Cargo.toml), is 1.88.0. However, you will get incorrect (false positive) `dead_code`
warnings, unless you use Rust 1.89.0+. (See
[rust-lang/rust#142208](https://github.com/rust-lang/rust/pull/142208).)

## Related issues

Please give thumbs up (and contribute, if you can) to

- [SergioBenitez/proc-macro2-diagnostics#13](https://github.com/SergioBenitez/proc-macro2-diagnostics/issues/13)
  defect: Error message and details missing, when macro fails to generate main() on STABLE
- [dtolnay/no-panic#28](https://github.com/dtolnay/no-panic/issues/28) no-panic currently cannot be
  used in doctests
- [dtolnay/no-panic#78](https://github.com/dtolnay/no-panic/issues/78) Not working (false negative)
  in integration tests
