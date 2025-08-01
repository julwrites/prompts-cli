use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::{tempdir, TempDir};
use std::fs;
use std::path::PathBuf;
use assert_cmd::cargo::CommandCargoExt;
use prompts_cli::{Prompt, storage::Storage};
use toml::Value;

struct CliTestEnv {
    _config_dir: TempDir,
    config_path: PathBuf,
    _prompts_storage_dir: TempDir,
    storage_path: PathBuf,
}

impl CliTestEnv {
    fn new() -> anyhow::Result<Self> {
        let config_dir = tempdir()?;
        let config_path = config_dir.path().join("config.toml");

        let prompts_storage_dir = tempdir()?;
        let prompts_storage_path = prompts_storage_dir.path().to_path_buf();

        let mut config = toml::map::Map::new();
        let mut storage_config = toml::map::Map::new();
        storage_config.insert(
            "path".to_string(),
            Value::String(prompts_storage_path.to_string_lossy().into_owned()),
        );
        config.insert("storage".to_string(), Value::Table(storage_config));

        let config_content = toml::to_string(&config)?;
        fs::write(&config_path, config_content)?;

        Ok(Self {
            _config_dir: config_dir,
            config_path,
            _prompts_storage_dir: prompts_storage_dir,
            storage_path: prompts_storage_path,
        })
    }
}

#[tokio::test]
async fn test_cli_import_export() -> anyhow::Result<()> {
    let env = CliTestEnv::new()?;

    // Create a source directory with some prompts
    let source_dir = tempdir()?;
    let mut prompt1 = Prompt::new("Prompt 1 content", None, None);
    let mut prompt2 = Prompt::new("Prompt 2 content", None, None);

    let storage_source =
        prompts_cli::storage::JsonStorage::new(Some(source_dir.path().to_path_buf()))?;
    storage_source.save_prompt(&mut prompt1)?;
    storage_source.save_prompt(&mut prompt2)?;

    // Test import
    let mut cmd = Command::cargo_bin(r#"prompts-cli"#)?;
    cmd.arg("--config")
        .arg(&env.config_path)
        .arg("import")
        .arg(source_dir.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Imported 2 prompts"));

    // Verify imported prompts
    let storage_dest = prompts_cli::storage::JsonStorage::new(Some(env.storage_path.clone()))?;
    let loaded_prompts = storage_dest.load_prompts()?;
    assert_eq!(loaded_prompts.len(), 2);
    assert!(loaded_prompts
        .iter()
        .any(|p| p.content == "Prompt 1 content"));
    assert!(loaded_prompts
        .iter()
        .any(|p| p.content == "Prompt 2 content"));

    // Test export
    let export_dir = tempdir()?;
    let mut cmd = Command::cargo_bin(r#"prompts-cli"#)?;
    cmd.arg("--config")
        .arg(&env.config_path)
        .arg("export")
        .arg(export_dir.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Exported 2 prompts"));

    // Verify exported prompts
    let exported_files: Vec<_> = fs::read_dir(export_dir.path())?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<_, std::io::Error>>()?;
    assert_eq!(exported_files.len(), 2);

    Ok(())
}
