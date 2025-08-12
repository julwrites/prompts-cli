use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::tempdir;
use std::fs;
use prompts_cli::{Prompt, Prompts, storage::JsonStorage};
use toml::Value;

#[tokio::test]
async fn test_cli_config_file() -> anyhow::Result<()> {
    let config_dir = tempdir()?;
    let config_path = config_dir.path().join("config.toml");

    // Create a dedicated temporary directory for prompts storage
    let prompts_storage_dir = tempdir()?;
    let prompts_storage_path = prompts_storage_dir.path();

    let mut config = toml::map::Map::new();
    let mut storage_config = toml::map::Map::new();
    storage_config.insert(
        "path".to_string(),
        Value::String(prompts_storage_path.to_string_lossy().into_owned()),
    );
    config.insert("storage".to_string(), Value::Table(storage_config));

    let config_content = toml::to_string(&config)?;
    fs::write(&config_path, config_content)?;

    // Add a prompt using the prompts_api directly to the configured storage path
    let storage = JsonStorage::new(Some(prompts_storage_path.to_path_buf()))?;
    let prompts_api = Prompts::new(Box::new(storage));
    let mut prompt = Prompt::new("Config test prompt content", None, None);
    prompts_api.add_prompt(&mut prompt).await?;

    let mut cmd = Command::cargo_bin("prompts-cli")?;
    cmd.arg("--config").arg(&config_path).arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&format!("{} - Config test prompt content", &prompt.hash[..12])));

    Ok(())
}

#[tokio::test]
async fn test_cli_default_config_location() -> anyhow::Result<()> {
    // 1. Create a temporary directory to act as the config directory.
    let temp_config_dir = tempdir()?;

    // 2. Create the nested `prompts-cli` directory structure.
    let config_dir = temp_config_dir.path();
    let prompts_cli_dir = config_dir.join("prompts-cli");
    fs::create_dir_all(&prompts_cli_dir)?;
    let config_path = prompts_cli_dir.join("config.toml");

    // 3. Create a dedicated temporary directory for prompts storage.
    let prompts_storage_dir = tempdir()?;
    let prompts_storage_path = prompts_storage_dir.path();

    // 4. Write a config file in the fake config directory.
    let mut config = toml::map::Map::new();
    let mut storage_config = toml::map::Map::new();
    storage_config.insert(
        "path".to_string(),
        Value::String(prompts_storage_path.to_string_lossy().into_owned()),
    );
    config.insert("storage".to_string(), Value::Table(storage_config));
    let config_content = toml::to_string(&config)?;
    fs::write(&config_path, config_content)?;

    // 5. Add a prompt directly to the storage path specified in the config.
    let storage = JsonStorage::new(Some(prompts_storage_path.to_path_buf()))?;
    let prompts_api = Prompts::new(Box::new(storage));
    let mut prompt = Prompt::new("Default location config test", None, None);
    prompts_api.add_prompt(&mut prompt).await?;

    // 6. Run the CLI, setting our new test-only env var.
    let mut cmd = Command::cargo_bin("prompts-cli")?;
    cmd.env("PROMPTS_CLI_CONFIG_DIR_FOR_TESTING", config_dir);
    // Unset other config-related env vars to ensure we are testing the correct mechanism.
    cmd.env_remove("PROMPTS_CLI_CONFIG_PATH");
    cmd.arg("list");

    // 7. Assert that the CLI finds the prompt, proving it used our config.
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&format!("{} - Default location config test", &prompt.hash[..12])));

    Ok(())
}

#[tokio::test]
async fn test_cli_default_config_file() -> anyhow::Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("config.toml");

    // Create a dedicated temporary directory for prompts storage
    let prompts_storage_dir = tempdir()?;
    let prompts_storage_path = prompts_storage_dir.path();

    let mut config = toml::map::Map::new();
    let mut storage_config = toml::map::Map::new();
    storage_config.insert(
        "path".to_string(),
        Value::String(prompts_storage_path.to_string_lossy().into_owned()),
    );
    config.insert("storage".to_string(), Value::Table(storage_config));

    let config_content = toml::to_string(&config)?;
    fs::write(&config_path, config_content)?;

    // Add a prompt using the prompts_api directly to the configured storage path
    let storage = JsonStorage::new(Some(prompts_storage_path.to_path_buf()))?;
    let prompts_api = Prompts::new(Box::new(storage));
    let mut prompt = Prompt::new("Default config test prompt content", None, None);
    prompts_api.add_prompt(&mut prompt).await?;

    let mut cmd = Command::cargo_bin("prompts-cli")?;
    cmd.env("PROMPTS_CLI_CONFIG_PATH", &config_path);
    cmd.arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&format!("{} - Default config test prompt content", &prompt.hash[..12])));

    Ok(())
}