#![doc = top_level!( )]
all_by_file!(top_level, "some-file.toml");

pub use readme_code_extractor_proc::{all, nth};

//readme_code_extractor_proc::assert_exact_version!{ "0.1.0" }

/// No need to be public.
const _ASSERT_README_CODE_EXTRACTOR_LIB_VERSION: () = {
    let proc_version = readme_code_extractor_proc::version!();

    if !is_exact_version(proc_version) {
        panic!("prudent-rs/readme-code-extractor-proc is of different version than prudent-rs/readme-code-extractor. Please report this as an issue, along with both versions.");
    }
};

// Rust below 1.89.0 reports the following function as "unused", even though it's used for
// `_ASSERT_README_CODE_EXTRACTOR_LIB_VERSION`. That's why `#[allow(dead_code)]`.
/// No need to be public.
#[allow(dead_code)]
const fn is_exact_version(expected_version: &'static str) -> bool {
    matches!(expected_version.as_bytes(), b"0.1.0")
}

/// No need to be public.
const _ASSERT_VERSION: () = {
    if !crate::is_exact_version(env!("CARGO_PKG_VERSION")) {
        panic!("prudent-rs/readme-code-extractor has its function is_exact_version() out of date.");
    }
};

/// Like [all], but load the configuration from a (TOML) file.
#[macro_export]
macro_rules! all_by_file {
    ( $name_of_new_extractor_macro:ident, $config_file_path:literal ) => {
        const _: () = {
            const _: &str = $config_file_path;
            // We have to use `include_str!`, so that any change to the config file gets noticed by
            // rustc/cargo, so that on next `cargo check` amd similar they rebuild the crate.
            ::core::hint::black_box(::core::include_str!($config_file_path));
        };

        // Create an extractor macro. In case of `all_by_file` we don't strictly need it, but then
        // the user would have to guarantee that she would
        // `::core::hint::black_box(::core::include_str!($config_file_path));` on her own. And that
        // allows for human error.
        //
        // (In case of `nth_by_file` we do need to create an extractor macro either way, because of
        // the extra parameter `n`. See `nth_by_file`.)
        //
        // We do HAVE TO `macro_export` it. Otherwise it can't be used in `#[doc = ... ]` or `#![doc
        // = ... ]` (which are processed in a crate separate to the crate that called `all_by_file`
        // macro).
        #[macro_export]
        macro_rules! $name_of_new_extractor_macro {
                    () => {
                        // Unfortunately, Rust macros are lazy (and not eager). So here we can't "just tell" or
                        // `all!` macro to "evaluate"/receive/get result of
                        // `::core::include_str!($config_file_path)`.
                        //
                        // Therefore we call a proc macro, which loads the file and passes its content to `all` macro
                        ::readme_code_extractor_proc::all!($config_file_path)
                    };
                }
    };
}

/// Like [nth], but load the configuration from a (TOML) file. See also [all_by_file].
#[macro_export]
macro_rules! nth_by_file {
    ( $name_of_new_extractor_macro:ident, $config_file_path:literal ) => {
        const _: () = {
            const _: &str = $config_file_path;
            ::core::hint::black_box(::core::include_str!($config_file_path));
        };

        // Unlike `all_by_file`, here we can't simply create an accessor/applier macro using
        // `macro_rules`. This accessor/applier macro needs to accept a parameter (position/index of
        // the code block to return).
        //
        // But, `macro_rules` can't create a macro that itself creates uses `macro_rules` to create
        // another macro that accepts parameters, because the dollar prefix would be resolved as if
        // it were a capturing variable from the outer macro. (And `$$` is not stable yet
        // (https://github.com/rust-lang/rust/issues/83527)).
        /*
        #[macro_export]
        macro_rules! $name_of_new_extractor_macro {
            ($$a:literal) => {
                ::readme_code_extractor_proc::nth!($config_file_path);
                let _ = $$a;
            };
        }*/
    };
}

#[macro_export]
macro_rules! generate {
    //    () => { "Gen" };
    () => {{
        const _: () = {};
        "Hi"
    }};
}
