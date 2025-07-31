use assert_cmd::Command;

#[test]
fn generates_qr_from_text() {
    let mut cmd = Command::cargo_bin("rsqr").unwrap();
    cmd.arg("Hello world");
    cmd.assert().success().stdout(predicates::str::contains("██"));
}

#[test]
fn png_file_is_created() {
    let file = "test_qr.png";
    let _ = std::fs::remove_file(file); // Clean up before

    let mut cmd = Command::cargo_bin("rsqr").unwrap();
    cmd.args(&["--png", file, "Test PNG"]);
    cmd.assert().success();

    assert!(std::path::Path::new(file).exists());
    let _ = std::fs::remove_file(file); // Clean up after
}
