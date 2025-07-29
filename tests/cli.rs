use std::fs::File;
use std::io::Write;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_cli_list() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("prompts.json");

    let mut file = File::create(&file_path).unwrap();
    file.write_all(b"[
  {
    \"name\": \"Test Prompt\",
    \"text\": \"This is a test prompt.\"
  }
]").unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", "list", "--file", file_path.to_str().unwrap()])
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    assert!(stdout.contains("Test Prompt: This is a test prompt."));
}

#[test]
fn test_cli_show() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("prompts.json");

    let mut file = File::create(&file_path).unwrap();
    file.write_all(b"[
  {
    \"name\": \"Test Prompt\",
    \"text\": \"This is a test prompt.\"
  }
]").unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", "show", "Test Prompt", "--file", file_path.to_str().unwrap()])
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    assert_eq!(stdout.trim(), "This is a test prompt.");
}

#[test]
fn test_cli_show_not_found() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("prompts.json");

    let mut file = File::create(&file_path).unwrap();
    file.write_all(b"[]").unwrap();

    let output = Command::new("cargo")
        .args(["run", "--", "show", "Non-existent Prompt", "--file", file_path.to_str().unwrap()])
        .output()
        .expect("failed to execute process");

    let stderr = String::from_utf8(output.stderr).unwrap();

    assert!(!output.status.success());
    assert!(stderr.contains("Prompt 'Non-existent Prompt' not found"));
}

