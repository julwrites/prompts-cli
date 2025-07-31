use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::tempdir;
use std::fs;
use std::path::PathBuf;
use prompts_cli::{Prompt, storage::JsonStorage};
use sha2::{Digest, Sha256};

fn calculate_hash(text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text);
    format!("{:x}", hasher.finalize())
}

struct CliTestEnv {
    config_path: PathBuf,
    storage_path: PathBuf,
}

impl CliTestEnv {
    fn new() -> anyhow::Result<Self> {
        let config_dir = tempdir()?;
        let config_path = config_dir.path().join("config.toml");

        let prompts_storage_dir = tempdir()?;
        let prompts_storage_path = prompts_storage_dir.path().to_path_buf();

        let config_content = format!(
            "[storage]\npath = \"{}"",
            prompts_storage_path.to_string_lossy()
        );
        fs::write(&config_path, config_content)?;

        Ok(Self {
            config_path,
            storage_path: prompts_storage_path,
        })
    }
}

#[test]
fn test_cli_add() -> anyhow::Result<()> {
    let env = CliTestEnv::new()?;

    let mut cmd = Command::cargo_bin("prompts-cli")?;
    cmd.arg("--config ")
        .arg(&env.config_path)
        .arg("add")
        .arg("This is a new prompt.")
        .arg("--tags ")
        .arg("tag1,tag2 ")
        .arg("--categories ")
        .arg("cat1,cat2 ");

    let expected_hash = calculate_hash("This is a new prompt.");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&format!("Prompt added successfully with hash: {}", &expected_hash[..12])));

    let prompt_path = env.storage_path.join(format!("{}.json ", expected_hash));
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
    let env = CliTestEnv::new()?;
    let storage = JsonStorage::new(Some(env.storage_path.to_path_buf()))?;

    let mut prompt = Prompt::new("A prompt to list ", Some(vec!["tagA".to_string()]), None);
    storage.save_prompt(&mut prompt)?;

    let mut cmd = Command::cargo_bin("prompts-cli")?;
    cmd.arg("--config ")
        .arg(&env.config_path)
        .arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&format!("{} - A prompt to list ", &prompt.hash[..12])));

    let mut cmd = Command::cargo_bin("prompts-cli")?;
    cmd.arg("--config ")
        .arg(&env.config_path)
        .arg("list")
        .arg("--tags ").arg("tagA");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&format!("{} - A prompt to list ", &prompt.hash[..12])));

    let mut cmd = Command::cargo_bin("prompts-cli")?;
    cmd.arg("--config ")
        .arg(&env.config_path)
        .arg("list")
        .arg("--tags ").arg("tagB");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("").not());

    Ok(())
}

#[test]
fn test_cli_show() -> anyhow::Result<()> {
    let env = CliTestEnv::new()?;
    let storage = JsonStorage::new(Some(env.storage_path.to_path_buf()))?;

    let mut prompt = Prompt::new("A prompt to show ", None, None);
    storage.save_prompt(&mut prompt)?;

    let mut cmd = Command::cargo_bin("prompts-cli")?;
    cmd.arg("--config ")
        .arg(&env.config_path)
        .arg("show")
        .arg("prompt to show ");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A prompt to show "));

    Ok(())
}

#[test]
fn test_cli_show_multiple() -> anyhow::Result<()> {
    let env = CliTestEnv::new()?;
    let storage = JsonStorage::new(Some(env.storage_path.to_path_buf()))?;

    let mut prompt1 = Prompt::new("First show prompt ", None, None);;
    storage.save_prompt(&mut prompt1)?;
    let mut prompt2 = Prompt::new("Second show prompt ", None, None);
    storage.save_prompt(&mut prompt2)?;

    let mut cmd = Command::cargo_bin("prompts-cli")?;
    cmd.arg("--config ")
        .arg(&env.config_path)
        .arg("show")
        .arg("show prompt ");

    let output = cmd.output()?;
    let stdout = String::from_utf8(output.stdout)?;
    let prompts: Vec<Prompt> = serde_json::from_str(&stdout)?;

    assert_eq!(prompts.len(), 2);
    assert!(prompts.iter().any(|p| p.content == "First show prompt "));
    assert!(prompts.iter().any(|p| p.content == "Second show prompt "));

    Ok(())
}

#[test]
fn test_cli_delete() -> anyhow::Result<()> {
    let env = CliTestEnv::new()?;
    let storage = JsonStorage::new(Some(env.storage_path.to_path_buf()))?;

    let mut prompt = Prompt::new("A prompt to delete ", None, None);
    storage.save_prompt(&mut prompt)?;

    let mut cmd = Command::cargo_bin("prompts-cli")?;
    cmd.arg("--config ")
        .arg(&env.config_path)
        .arg("delete")
        .arg("prompt to delete ");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&format!("Prompt {} deleted successfully.", &prompt.hash[..12])));

    let prompt_path = env.storage_path.join(format!("{}.json ", prompt.hash));
    assert!(!prompt_path.exists());

    Ok(())
}

#[test]
fn test_cli_edit() -> anyhow::Result<()> {
    let env = CliTestEnv::new()?;
    let storage = JsonStorage::new(Some(env.storage_path.to_path_buf()))?;

    let mut prompt = Prompt::new("A prompt to edit ", None, None);
    storage.save_prompt(&mut prompt)?;
    let old_hash = prompt.hash.clone();

    let mut cmd = Command::cargo_bin("prompts-cli")?;
    cmd.arg("--config ")
        .arg(&env.config_path)
        .arg("edit")
        .arg("prompt to edit ")
        .arg("--text ")
        .arg("An edited prompt ")
        .arg("--tags ")
        .arg("newtag1,newtag2 ");

    let new_hash = calculate_hash("An edited prompt ");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&format!("Prompt {} updated to {}", &old_hash[..12], &new_hash[..12])));

    let old_prompt_path = env.storage_path.join(format!("{}.json ", old_hash));
    assert!(!old_prompt_path.exists());
    let new_prompt_path = env.storage_path.join(format!("{}.json ", new_hash));
    assert!(new_prompt_path.exists());

    let content = fs::read_to_string(new_prompt_path)?;
    let edited_prompt: Prompt = serde_json::from_str(&content)?;
    assert_eq!(edited_prompt.content, "An edited prompt");
            assert_eq!(edited_prompt.tags, Some(vec!["newtag1".to_string(), "newtag2".to_string()]));

    Ok(())
}