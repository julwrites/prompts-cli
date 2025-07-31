use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::tempdir;
use std::fs;

#[test]
fn test_cli_config_file() -> anyhow::Result<()> {
    let config_dir = tempdir()?;
    let config_path = config_dir.path().join("config.toml");

    // Create a dedicated temporary directory for prompts storage
    let prompts_storage_dir = tempdir()?;
    let prompts_storage_path = prompts_storage_dir.path();

    let config_content = format!(
        "[storage]\npath = \"{}\"",
        prompts_storage_path.to_string_lossy()
    );
    fs::write(&config_path, config_content)?;

    let mut cmd = Command::cargo_bin("prompts-cli")?;
    cmd.arg("--config").arg(&config_path).arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("List command"));

    Ok(())
}
