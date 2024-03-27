// tests/cli_test.rs
use std::process::Command;

#[test]
fn test_vividtext_output() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("your example text")
        .output()
        .expect("failed to execute process");

    let output_str = String::from_utf8(output.stdout).unwrap();

    // ここでoutput_strに対して期待する出力が含まれているかチェック
    // 例えば、グラデーションが正しく適用されているかなど
    assert!(output_str.contains("expected output part"));
}
