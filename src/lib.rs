// @TODO
//
//#![doc = top_level!( )]
//all_by_file!(top_level, "some-file.toml");

pub use readme_code_extractor_proc::{all, all_by_file, nth};

/// We can't report the actual version(s), because [panic] macro is not eager, and passing in
/// (formatting) variables doesn't work in const context. See also
/// <https://rustc-dev-guide.rust-lang.org/macro-expansion.html#eager-expansion>.
const _: () = {
    let proc_version = readme_code_extractor_proc::version!();

    if !is_exact_version(proc_version) {
        panic!(
            "prudent-rs/readme-code-extractor-proc is of different version than \
                prudent-rs/readme-code-extractor. Please report this as an issue, along with both \
                versions."
        );
    }
};

const fn is_exact_version(expected_version: &'static str) -> bool {
    // We can't use a comparison operator ==, because trait PartialEq is not const (in April 2026).
    matches!(expected_version.as_bytes(), b"0.0.1")
}

const _: () = {
    if !crate::is_exact_version(env!("CARGO_PKG_VERSION")) {
        panic!("prudent-rs/readme-code-extractor has its function is_exact_version() out of date.");
    }
};
