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
    // 1. Create a fake home directory.
    let fake_home = tempdir()?;

    // 2. Determine the platform-specific config directory relative to fake_home.
    let config_parent_dir = if cfg!(target_os = "windows") {
        // On Windows, the config is typically in AppData\Roaming
        fake_home.path().join("AppData/Roaming")
    } else if cfg!(target_os = "macos") {
        // On macOS, it's in Library/Application Support
        fake_home.path().join("Library/Application Support")
    } else {
        // On Linux and other Unix-like systems, it's in .config
        fake_home.path().join(".config")
    };
    let config_dir = config_parent_dir.join("prompts-cli");
    fs::create_dir_all(&config_dir)?;
    let config_path = config_dir.join("config.toml");

    // 3. Create a dedicated temporary directory for prompts storage.
    let prompts_storage_dir = tempdir()?;
    let prompts_storage_path = prompts_storage_dir.path();

    // 4. Write a config file in the fake home directory.
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

    // 6. Run the CLI, setting HOME to our fake home directory.
    let mut cmd = Command::cargo_bin("prompts-cli")?;
    cmd.env("HOME", fake_home.path());
    // Unset other config-related env vars to ensure the test is hermetic.
    cmd.env_remove("PROMPTS_CLI_CONFIG_PATH");
    cmd.env_remove("XDG_CONFIG_HOME");
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