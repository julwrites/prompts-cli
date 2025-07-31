use prompts_cli::{Prompt, Prompts, storage::JsonStorage};
use tempfile::tempdir;

#[tokio::test]
async fn test_generate_command() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let storage_path = dir.path().to_path_buf();
    let storage = JsonStorage::new(Some(storage_path.clone()))?;
    let prompts_api = Prompts::new(Box::new(storage));

    let mut prompt = Prompt::new("Hello, {{name}}!", None, None);
    prompts_api.add_prompt(&mut prompt).await?;

    let mut cmd = assert_cmd::Command::cargo_bin("prompts-cli")?;
    cmd.arg("--config")
        .arg(storage_path)
        .arg("generate")
        .arg("Hello")
        .arg("--variables")
        .arg("name=World");

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Hello, World!"));

    Ok(())
}
