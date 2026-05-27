#[cfg(debug_assertions)]
compile_error!("Build in release mode only, with `cargo build --release`.");

#[cfg(test)]
compile_error!("Don't test. Instead, build and in release, that is, with `cargo build --release`.");
