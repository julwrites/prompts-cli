use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::NamedTempFile;
use std::io::Write;
use prompts_core::{Prompt, load_prompts};
use std::fs;

#[test]
fn test_cli_list() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, r#"[
  {{
    "name": "Test Prompt",
    "text": "This is a test prompt.",
    "tags": [],
    "categories": []
  }}
]"#)?;

    let mut cmd = Command::cargo_bin("prompts-cli")?;
    cmd.arg("--file")
        .arg(file.path())
        .arg("list");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Test Prompt: This is a test prompt."));

    Ok(())
}

#[test]
fn test_cli_show() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, r#"[
  {{
    "name": "Test Prompt",
    "text": "This is a test prompt.",
    "tags": [],
    "categories": []
  }}
]"#)?;

    let mut cmd = Command::cargo_bin("prompts-cli")?;
    cmd.arg("--file")
        .arg(file.path())
        .arg("show")
        .arg("Test Prompt");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("This is a test prompt."));

    Ok(())
}

#[test]
fn test_cli_show_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "[]")?;

    let mut cmd = Command::cargo_bin("prompts-cli")?;
    cmd.arg("--file")
        .arg(file.path())
        .arg("show")
        .arg("Non-existent Prompt");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Prompt 'Non-existent Prompt' not found"));

    Ok(())
}

#[test]
fn test_cli_generate() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, r#"[
  {{
    "name": "Test Prompt",
    "text": "This is a test prompt.",
    "tags": [],
    "categories": []
  }}
]"#)?;

    let mut cmd = Command::cargo_bin("prompts-cli")?;
    cmd.arg("--file")
        .arg(file.path())
        .arg("generate")
        .arg("Test Prompt")
        .arg("--generator")
        .arg("mock");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Generated text for 'This is a test prompt.'"));

    Ok(())
}

#[test]
fn test_cli_add() -> Result<(), Box<dyn std::error::Error>> {
    let file = NamedTempFile::new()?;

    let mut cmd = Command::cargo_bin("prompts-cli")?;
    cmd.arg("--file")
        .arg(file.path())
        .arg("add")
        .arg("New Prompt")
        .arg("This is a new prompt.")
        .arg("--tags")
        .arg("tag1,tag2")
        .arg("--categories")
        .arg("cat1,cat2");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Prompt 'New Prompt' added successfully."));

    let prompts_content = fs::read_to_string(file.path())?;
    let prompts: Vec<Prompt> = serde_json::from_str(&prompts_content)?;

    assert_eq!(prompts.len(), 1);
    let new_prompt = &prompts[0];
    assert_eq!(new_prompt.name, "New Prompt");
    assert_eq!(new_prompt.text, "This is a new prompt.");
    assert_eq!(new_prompt.tags, vec!["tag1", "tag2"]);
    assert_eq!(new_prompt.categories, vec!["cat1", "cat2"]);

    Ok(())
}

#[test]
fn test_edit_prompt() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    let prompts = vec![
        Prompt {
            name: "test_prompt".to_string(),
            text: "This is a test prompt.".to_string(),
            tags: vec!["tag1".to_string()],
            categories: vec!["cat1".to_string()],
        },
    ];
    let json = serde_json::to_string(&prompts)?;
    writeln!(file, "{}", json)?;

    let mut cmd = Command::cargo_bin("prompts-cli")?;
    cmd.arg("--file")
        .arg(file.path())
        .arg("edit")
        .arg("test_prompt")
        .arg("--text")
        .arg("This is the edited text.")
        .arg("--tags")
        .arg("tag1,tag2")
        .arg("--categories")
        .arg("cat1,cat2");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Prompt 'test_prompt' updated successfully."));

    let updated_prompts = load_prompts(file.path().to_str().unwrap())?;
    let updated_prompt = updated_prompts.iter().find(|p| p.name == "test_prompt").unwrap();

    assert_eq!(updated_prompt.text, "This is the edited text.");
    assert_eq!(updated_prompt.tags, vec!["tag1".to_string(), "tag2".to_string()]);
    assert_eq!(updated_prompt.categories, vec!["cat1".to_string(), "cat2".to_string()]);

    Ok(())
}

#[test]
fn test_delete_prompt() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    let prompts = vec![
        Prompt {
            name: "test_prompt".to_string(),
            text: "This is a test prompt.".to_string(),
            tags: vec!["tag1".to_string()],
            categories: vec!["cat1".to_string()],
        },
    ];
    let json = serde_json::to_string(&prompts)?;
    writeln!(file, "{}", json)?;

    let mut cmd = Command::cargo_bin("prompts-cli")?;
    cmd.arg("--file")
        .arg(file.path())
        .arg("delete")
        .arg("test_prompt");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Prompt 'test_prompt' deleted successfully."));

    let updated_prompts = load_prompts(file.path().to_str().unwrap())?;
    assert!(updated_prompts.is_empty());

    Ok(())
}