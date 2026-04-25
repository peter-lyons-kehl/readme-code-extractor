#[test]
fn simple_no_panic() {
    let t = trybuild::TestCases::new();
    t.compile_fail("../negative_tests/src/bin/simple_no_panic.rs");
}