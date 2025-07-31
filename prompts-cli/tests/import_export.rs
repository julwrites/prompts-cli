use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::tempdir;
use std::fs;
use prompts_cli::{Prompt, storage::{Storage, JsonStorage}};

#[tokio::test]
async fn test_cli_import_export() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let storage_path = dir.path().to_path_buf();

    // Create a source directory with some prompts
    let source_dir = tempdir()?;
    let mut prompt1 = Prompt::new("Prompt 1 content", None, None);
    let mut prompt2 = Prompt::new("Prompt 2 content", None, None);

    let storage_source = prompts_cli::storage::JsonStorage::new(Some(source_dir.path().to_path_buf()))?;
    storage_source.save_prompt(&mut prompt1)?;
    storage_source.save_prompt(&mut prompt2)?;

    // Test import
    let mut cmd = Command::cargo_bin("prompts-cli")?;
    cmd.arg("--config")
        .arg(&storage_path)
        .arg("import")
        .arg(source_dir.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Imported 2 prompts"));

    // Verify imported prompts
    let storage_dest = prompts_cli::storage::JsonStorage::new(Some(storage_path.clone()))?;
    let loaded_prompts = storage_dest.load_prompts()?;
    assert_eq!(loaded_prompts.len(), 2);
    assert!(loaded_prompts.iter().any(|p| p.content == "Prompt 1 content"));
    assert!(loaded_prompts.iter().any(|p| p.content == "Prompt 2 content"));

    // Test export
    let export_dir = tempdir()?;
    let mut cmd = Command::cargo_bin("prompts-cli")?;
    cmd.arg("--config")
        .arg(&storage_path)
        .arg("export")
        .arg(export_dir.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Exported 2 prompts"));

    // Verify exported prompts
    let exported_files: Vec<_> = fs::read_dir(export_dir.path())?.map(|res| res.map(|e| e.path())).collect::<Result<_, std::io::Error>>()?;
    assert_eq!(exported_files.len(), 2);

    Ok(())
}
