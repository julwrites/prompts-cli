use prompts_cli::{Prompt, Prompts, storage::JsonStorage};
use tempfile::{tempdir, TempDir};
use std::path::PathBuf;
use std::fs;

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

        let config_content = format!(
            "[storage]\npath = \"{}\"",
            prompts_storage_path.to_string_lossy()
        );
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
async fn test_generate_command() -> anyhow::Result<()> {
    let env = CliTestEnv::new()?;
    let storage = JsonStorage::new(Some(env.storage_path.to_path_buf()))?;
    let prompts_api = Prompts::new(Box::new(storage));

    let mut prompt = Prompt::new("Hello, {{name}}!", None, None);
    prompts_api.add_prompt(&mut prompt).await?;

    let mut cmd = assert_cmd::Command::cargo_bin(r#"prompts-cli"#)?;
    cmd.arg("--config")
        .arg(&env.config_path)
        .arg("generate")
        .arg("Hello")
        .arg("--variables")
        .arg("name=World");

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Hello, World!"));

    Ok(())
}

