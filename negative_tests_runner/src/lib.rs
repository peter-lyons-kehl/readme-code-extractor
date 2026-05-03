#[cfg(test)]
mod tests {
    use core::error::Error;
    use snapbox::cmd::Command;

    #[test]
    fn simple_no_panic() -> Result<(), Box<dyn Error>> {
        // Do NOT set arg("--release") - build in debug mode only. Otherwise, since we have multiple
        // sequential parts where each fails with panic!, a release build would optimize the
        // subsequent panics out. Then the error message(s) would not match.
        //
        // That is also enforced by ../negative_tests/src/lib.rs
        let cmd = Command::new("cargo").arg("build");
        let cmd = cmd.current_dir(std::fs::canonicalize("../negative_tests")?);

        let assert = cmd.assert().failure();

        // Using stderr_eq is less readable than multiple stderr.contains(...)
        /*assert.stderr_eq(
            "...\n\
             [..]ERROR[no-panic]: detected panic in function `main`[..]\n\
             ...\n\
             [..]ERROR[no-panic]: detected panic in function `divide`[..]\n\
             ...\n\
             [..]ERROR[no-panic]: detected panic in function `slice_access`[..]\n\
             ...\n",
        );*/
        let output = assert.get_output();
        let stderr: &str = str::from_utf8(&output.stderr)?;

        assert!(stderr.contains("ERROR[no-panic]: detected panic in function `main`"));
        assert!(stderr.contains("ERROR[no-panic]: detected panic in function `divide`"));
        assert!(stderr.contains("ERROR[no-panic]: detected panic in function `slice_access`"));
        Ok(())
    }
}
