use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::tempdir;
use prompts_cli::{Prompt, storage::{Storage, JsonStorage}};
use std::fs;
use sha2::{Digest, Sha256};

fn calculate_hash(text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text);
    format!("{:x}", hasher.finalize())
}

#[test]
fn test_cli_add() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let storage_path = dir.path();

    let mut cmd = Command::cargo_bin("prompts-cli")?;
    cmd.arg("--config")
        .arg(storage_path)
        .arg("add")
        .arg("This is a new prompt.")
        .arg("--tags")
        .arg("tag1,tag2")
        .arg("--categories")
        .arg("cat1,cat2");

    let expected_hash = calculate_hash("This is a new prompt.");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&format!("Prompt added successfully with hash: {}", &expected_hash[..12])));

    let prompt_path = storage_path.join(format!("{}.json", expected_hash));
    assert!(prompt_path.exists());

    let content = fs::read_to_string(prompt_path)?;
    let prompt: Prompt = serde_json::from_str(&content)?;
    assert_eq!(prompt.content, "This is a new prompt.");
    assert_eq!(prompt.tags, Some(vec!["tag1".to_string(), "tag2".to_string()]));
    assert_eq!(prompt.categories, Some(vec!["cat1".to_string(), "cat2".to_string()]));

    Ok(())
}

#[test]
fn test_cli_list() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let storage_path = dir.path();
    let storage = JsonStorage::new(Some(storage_path.to_path_buf()))?;

    let mut prompt = Prompt::new("A prompt to list", None, None);
    storage.save_prompt(&mut prompt)?;

    let mut cmd = Command::cargo_bin("prompts-cli")?;
    cmd.arg("--config")
        .arg(storage_path)
        .arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&format!("{} - A prompt to list", &prompt.hash[..12])));

    Ok(())
}

#[test]
fn test_cli_show() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let storage_path = dir.path();
    let storage = JsonStorage::new(Some(storage_path.to_path_buf()))?;

    let mut prompt = Prompt::new("A prompt to show", None, None);
    storage.save_prompt(&mut prompt)?;

    let mut cmd = Command::cargo_bin("prompts-cli")?;
    cmd.arg("--config")
        .arg(storage_path)
        .arg("show")
        .arg("prompt to show");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A prompt to show"));

    Ok(())
}

#[test]
fn test_cli_show_multiple() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let storage_path = dir.path();
    let storage = JsonStorage::new(Some(storage_path.to_path_buf()))?;

    let mut prompt1 = Prompt::new("First show prompt", None, None);
    storage.save_prompt(&mut prompt1)?;
    let mut prompt2 = Prompt::new("Second show prompt", None, None);
    storage.save_prompt(&mut prompt2)?;

    let mut cmd = Command::cargo_bin("prompts-cli")?;
    cmd.arg("--config")
        .arg(storage_path)
        .arg("show")
        .arg("show prompt");

    let output = cmd.output()?;
    let stdout = String::from_utf8(output.stdout)?;
    let prompts: Vec<Prompt> = serde_json::from_str(&stdout)?;

    assert_eq!(prompts.len(), 2);
    assert!(prompts.iter().any(|p| p.content == "First show prompt"));
    assert!(prompts.iter().any(|p| p.content == "Second show prompt"));

    Ok(())
}

#[test]
fn test_cli_delete() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let storage_path = dir.path();
    let storage = JsonStorage::new(Some(storage_path.to_path_buf()))?;

    let mut prompt = Prompt::new("A prompt to delete", None, None);
    storage.save_prompt(&mut prompt)?;

    let mut cmd = Command::cargo_bin("prompts-cli")?;
    cmd.arg("--config")
        .arg(storage_path)
        .arg("delete")
        .arg("prompt to delete");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&format!("Prompt {} deleted successfully.", &prompt.hash[..12])));

    let prompt_path = storage_path.join("prompts").join(format!("{}.json", prompt.hash));
    assert!(!prompt_path.exists());

    Ok(())
}

#[test]
fn test_cli_edit() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let storage_path = dir.path();
    let storage = JsonStorage::new(Some(storage_path.to_path_buf()))?;

    let mut prompt = Prompt::new("A prompt to edit", None, None);
    storage.save_prompt(&mut prompt)?;
    let old_hash = prompt.hash.clone();

    let mut cmd = Command::cargo_bin("prompts-cli")?;
    cmd.arg("--config")
        .arg(storage_path)
        .arg("edit")
        .arg("prompt to edit")
        .arg("--text")
        .arg("An edited prompt");

    let new_hash = calculate_hash("An edited prompt");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&format!("Prompt {} updated to {}", &old_hash[..12], &new_hash[..12])));

    let old_prompt_path = storage_path.join("prompts").join(format!("{}.json", old_hash));
    assert!(!old_prompt_path.exists());
    let new_prompt_path = storage_path.join(format!("{}.json", new_hash));
    assert!(new_prompt_path.exists());

    let content = fs::read_to_string(new_prompt_path)?;
    let edited_prompt: Prompt = serde_json::from_str(&content)?;
    assert_eq!(edited_prompt.content, "An edited prompt");

    Ok(())
}
