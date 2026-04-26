#[cfg(test)]
mod tests {
    use core::error::Error;
    use snapbox::cmd::Command;

    #[test]
    fn simple_no_panic() -> Result<(), Box<dyn Error>> {
        // Do NOT set arg("--release") if having multiple sequential parts where each fails with
        // panic!. Otherwise release build will optimize the subsequent panics out! This is also
        // enforced by ../negative_tests/src/lib.rs
        let cmd = Command::new("cargo").arg("build");
        let cmd = cmd.current_dir(std::fs::canonicalize("../negative_tests")?);
        
        let assert = cmd.assert().failure();
        let output = assert.get_output();
        let stderr: &str = str::from_utf8( &output.stderr )?;

        assert!( stderr.contains("ERROR[no-panic]: detected panic in function `divide`"));
        assert!( stderr.contains("ERROR[no-panic]: detected panic in function `slice_access`"));

        Ok(())
    }
}
