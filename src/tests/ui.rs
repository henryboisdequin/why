#[test]
fn run_ui_tests() {
    let t = trybuild::TestCases::new();
    t.pass("src/tests/ui/simple_error.rs");
}
