// See also ../../negative_tests_runner/src/lib.rs.
#[cfg(not(debug_assertions))]
compile_error!(
    "Build in debug mode only. Otherwise, since we have multiple sequential parts \
                where each fails with panic!, a release build would optimize the \
                subsequent panics out. Then the error message(s) would not match."
);

#[cfg(test)]
compile_error!(
    "Don't test. Instead, build (in debug, that is, with `cargo build` or `cargo build --debug`."
);
