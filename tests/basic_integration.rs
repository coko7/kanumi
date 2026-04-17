// Basic integration test for binary invocation
// These tests can be expanded with assert_cmd or std::process::Command

#[test]
fn test_sanity_cli_invocation() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_kanumi"))
        .arg("--help")
        .output()
        .expect("failed to run kanumi binary");
    assert!(output.status.success(), "kanumi --help should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("kanumi"));
}
