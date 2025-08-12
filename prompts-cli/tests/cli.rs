use assert_cmd::prelude::*;
use predicates::prelude::*;
use prompts_cli::{Prompt, storage::{JsonStorage, LibSQLStorage, Storage}};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::tempdir;
use toml::Value;

fn calculate_hash(text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text);
    format!("{:x}", hasher.finalize())
}

use tempfile::TempDir;

struct CliTestEnv {
    _config_dir: TempDir,
    config_path: PathBuf,
    _storage_temp: TempDir,
    storage_path: PathBuf,
}

impl CliTestEnv {
    fn new(storage_type: &str) -> anyhow::Result<Self> {
        let config_dir = tempdir()?;
        let config_path = config_dir.path().join("config.toml");

        let storage_temp = tempdir()?;
        let storage_path = if storage_type == "json" {
            storage_temp.path().to_path_buf()
        } else {
            storage_temp.path().join("test.db")
        };

        let mut config = toml::map::Map::new();
        let mut storage_config = toml::map::Map::new();
        storage_config.insert("type".to_string(), Value::String(storage_type.to_string()));
        storage_config.insert(
            "path".to_string(),
            Value::String(storage_path.to_string_lossy().into_owned()),
        );
        config.insert("storage".to_string(), Value::Table(storage_config));

        let config_content = toml::to_string(&config)?;
        fs::write(&config_path, config_content)?;

        Ok(Self {
            _config_dir: config_dir,
            config_path,
            _storage_temp: storage_temp,
            storage_path,
        })
    }
}


async fn test_cli_add_impl(storage_type: &str) -> anyhow::Result<()> {
    let env = CliTestEnv::new(storage_type)?;

    let mut cmd = Command::cargo_bin(r#"prompts-cli"#)?;
    cmd.arg("--config")
        .arg(&env.config_path)
        .arg("add")
        .arg("This is a new prompt.")
        .arg("--tags")
        .arg("tag1,tag2")
        .arg("--categories")
        .arg("cat1,cat2");

    let expected_hash = calculate_hash("This is a new prompt.");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&format!(
            "Prompt added successfully with hash: {}",
            &expected_hash[..12]
        )));

    if storage_type == "json" {
        let prompt_path = env.storage_path.join(format!("{}.json", expected_hash));
        assert!(prompt_path.exists());
        let content = fs::read_to_string(prompt_path)?;
        let prompt: Prompt = serde_json::from_str(&content)?;
        assert_eq!(prompt.content, "This is a new prompt.");
        assert_eq!(
            prompt.tags,
            Some(vec!["tag1".to_string(), "tag2".to_string()])
        );
        assert_eq!(
            prompt.categories,
            Some(vec!["cat1".to_string(), "cat2".to_string()])
        );
    } else {
        let storage = prompts_cli::storage::LibSQLStorage::new(Some(env.storage_path)).await?;
        let prompts = storage.load_prompts().await?;
        let prompt = prompts.iter().find(|p| p.hash == expected_hash).unwrap();
        assert_eq!(prompt.content, "This is a new prompt.");
        assert_eq!(
            prompt.tags,
            Some(vec!["tag1".to_string(), "tag2".to_string()])
        );
        assert_eq!(
            prompt.categories,
            Some(vec!["cat1".to_string(), "cat2".to_string()])
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_cli_add_json() -> anyhow::Result<()> {
    test_cli_add_impl("json").await
}

#[tokio::test]
async fn test_cli_add_libsql() -> anyhow::Result<()> {
    test_cli_add_impl("libsql").await
}

async fn test_cli_add_duplicate_impl(storage_type: &str) -> anyhow::Result<()> {
    let env = CliTestEnv::new(storage_type)?;

    let mut cmd = Command::cargo_bin(r#"prompts-cli"#)?;
    cmd.arg("--config")
        .arg(&env.config_path)
        .arg("add")
        .arg("This is a duplicate prompt.");

    // First add
    cmd.assert().success();

    // Second add
    let mut cmd2 = Command::cargo_bin(r#"prompts-cli"#)?;
    cmd2.arg("--config")
        .arg(&env.config_path)
        .arg("add")
        .arg("This is a duplicate prompt.");

    cmd2.assert()
        .success()
        .stdout(predicate::str::contains("Prompt already exists."));

    Ok(())
}

#[tokio::test]
async fn test_cli_add_duplicate_json() -> anyhow::Result<()> {
    test_cli_add_duplicate_impl("json").await
}

#[tokio::test]
async fn test_cli_add_duplicate_libsql() -> anyhow::Result<()> {
    test_cli_add_duplicate_impl("libsql").await
}

async fn test_cli_list_impl(storage_type: &str) -> anyhow::Result<()> {
    let env = CliTestEnv::new(storage_type)?;
    let storage: Box<dyn Storage + Send + Sync> = if storage_type == "json" {
        Box::new(JsonStorage::new(Some(env.storage_path.to_path_buf()))?)
    } else {
        Box::new(LibSQLStorage::new(Some(env.storage_path.to_path_buf())).await?)
    };

    let mut prompt = Prompt::new("A prompt to list", Some(vec!["tagA".to_string()]), None);
    storage.save_prompt(&mut prompt).await?;

    let mut cmd = Command::cargo_bin(r#"prompts-cli"#)?;
    cmd.arg("--config").arg(&env.config_path).arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&format!(
            "{} - A prompt to list",
            &prompt.hash[..12]
        )));

    Ok(())
}

#[tokio::test]
async fn test_cli_list_json() -> anyhow::Result<()> {
    test_cli_list_impl("json").await
}

#[tokio::test]
async fn test_cli_list_libsql() -> anyhow::Result<()> {
    test_cli_list_impl("libsql").await
}

async fn test_cli_list_with_tag_filter_impl(storage_type: &str) -> anyhow::Result<()> {
    let env = CliTestEnv::new(storage_type)?;
    let storage: Box<dyn Storage + Send + Sync> = if storage_type == "json" {
        Box::new(JsonStorage::new(Some(env.storage_path.to_path_buf()))?)
    } else {
        Box::new(LibSQLStorage::new(Some(env.storage_path.to_path_buf())).await?)
    };

    let mut prompt1 = Prompt::new("A prompt with tag", Some(vec!["test-tag".to_string()]), None);
    storage.save_prompt(&mut prompt1).await?;

    let mut prompt2 = Prompt::new("A prompt without tag", None, None);
    storage.save_prompt(&mut prompt2).await?;

    let mut cmd = Command::cargo_bin(r#"prompts-cli"#)?;
    cmd.arg("--config")
        .arg(&env.config_path)
        .arg("list")
        .arg("--tags")
        .arg("test-tag");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A prompt with tag"))
        .stdout(predicate::str::contains("A prompt without tag").not());

    Ok(())
}

#[tokio::test]
async fn test_cli_list_with_tag_filter_json() -> anyhow::Result<()> {
    test_cli_list_with_tag_filter_impl("json").await
}

#[tokio::test]
async fn test_cli_list_with_tag_filter_libsql() -> anyhow::Result<()> {
    test_cli_list_with_tag_filter_impl("libsql").await
}

async fn test_cli_show_impl(storage_type: &str) -> anyhow::Result<()> {
    let env = CliTestEnv::new(storage_type)?;
    let storage: Box<dyn Storage + Send + Sync> = if storage_type == "json" {
        Box::new(JsonStorage::new(Some(env.storage_path.to_path_buf()))?)
    } else {
        Box::new(LibSQLStorage::new(Some(env.storage_path.to_path_buf())).await?)
    };

    let mut prompt = Prompt::new("A prompt to show", None, None);
    storage.save_prompt(&mut prompt).await?;

    let mut cmd = Command::cargo_bin(r#"prompts-cli"#)?;
    cmd.arg("--config")
        .arg(&env.config_path)
        .arg("show")
        .arg("prompt to show");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A prompt to show"));

    Ok(())
}

#[tokio::test]
async fn test_cli_show_json() -> anyhow::Result<()> {
    test_cli_show_impl("json").await
}

#[tokio::test]
async fn test_cli_show_libsql() -> anyhow::Result<()> {
    test_cli_show_impl("libsql").await
}

async fn test_cli_show_multiple_impl(storage_type: &str) -> anyhow::Result<()> {
    let env = CliTestEnv::new(storage_type)?;
    let storage: Box<dyn Storage + Send + Sync> = if storage_type == "json" {
        Box::new(JsonStorage::new(Some(env.storage_path.to_path_buf()))?)
    } else {
        Box::new(LibSQLStorage::new(Some(env.storage_path.to_path_buf())).await?)
    };

    let mut prompt1 = Prompt::new("First show prompt", None, None);
    storage.save_prompt(&mut prompt1).await?;
    let mut prompt2 = Prompt::new("Second show prompt", None, None);
    storage.save_prompt(&mut prompt2).await?;

    let mut cmd = Command::cargo_bin(r#"prompts-cli"#)?;
    cmd.arg("--config")
        .arg(&env.config_path)
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

#[tokio::test]
async fn test_cli_show_multiple_json() -> anyhow::Result<()> {
    test_cli_show_multiple_impl("json").await
}

#[tokio::test]
async fn test_cli_show_multiple_libsql() -> anyhow::Result<()> {
    test_cli_show_multiple_impl("libsql").await
}

async fn test_cli_delete_impl(storage_type: &str) -> anyhow::Result<()> {
    let env = CliTestEnv::new(storage_type)?;
    let storage: Box<dyn Storage + Send + Sync> = if storage_type == "json" {
        Box::new(JsonStorage::new(Some(env.storage_path.to_path_buf()))?)
    } else {
        Box::new(LibSQLStorage::new(Some(env.storage_path.to_path_buf())).await?)
    };

    let mut prompt = Prompt::new("A prompt to delete", None, None);
    let prompt_hash = prompt.hash.clone();
    storage.save_prompt(&mut prompt).await?;

    let mut cmd = Command::cargo_bin(r#"prompts-cli"#)?;
    cmd.arg("--config")
        .arg(&env.config_path)
        .arg("delete")
        .arg("prompt to delete");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&format!(
            "Prompt {} deleted successfully.",
            &prompt.hash[..12]
        )));

    if storage_type == "json" {
        let prompt_path = env.storage_path.join(format!("{}.json", prompt.hash));
        assert!(!prompt_path.exists());
    } else {
        let prompts = storage.load_prompts().await?;
        assert!(prompts.iter().find(|p| p.hash == prompt_hash).is_none());
    }


    Ok(())
}

#[tokio::test]
async fn test_cli_delete_json() -> anyhow::Result<()> {
    test_cli_delete_impl("json").await
}

#[tokio::test]
async fn test_cli_delete_libsql() -> anyhow::Result<()> {
    test_cli_delete_impl("libsql").await
}

async fn test_cli_edit_impl(storage_type: &str) -> anyhow::Result<()> {
    let env = CliTestEnv::new(storage_type)?;
    let storage: Box<dyn Storage + Send + Sync> = if storage_type == "json" {
        Box::new(JsonStorage::new(Some(env.storage_path.to_path_buf()))?)
    } else {
        Box::new(LibSQLStorage::new(Some(env.storage_path.to_path_buf())).await?)
    };

    let mut prompt = Prompt::new("A prompt to edit", None, None);
    storage.save_prompt(&mut prompt).await?;
    let old_hash = prompt.hash.clone();

    let mut cmd = Command::cargo_bin(r#"prompts-cli"#)?;
    cmd.arg("--config")
        .arg(&env.config_path)
        .arg("edit")
        .arg("prompt to edit")
        .arg("--text")
        .arg("An edited prompt")
        .arg("--add-tags")
        .arg("newtag1,newtag2");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&format!(
            "Prompt {} updated.",
            &old_hash[..12]
        )));

    if storage_type == "json" {
        let old_prompt_path = env.storage_path.join(format!("{}.json", old_hash));
        assert!(!old_prompt_path.exists());
        let prompts = storage.load_prompts().await?;
        let edited_prompt = prompts.iter().find(|p| p.content == "An edited prompt").unwrap();
        assert_eq!(
            edited_prompt.tags,
            Some(vec!["newtag1".to_string(), "newtag2".to_string()])
        );
    } else {
        let prompts = storage.load_prompts().await?;
        assert!(prompts.iter().find(|p| p.hash == old_hash).is_none());
        let edited_prompt = prompts.iter().find(|p| p.content == "An edited prompt").unwrap();
        assert_eq!(
            edited_prompt.tags,
            Some(vec!["newtag1".to_string(), "newtag2".to_string()])
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_cli_edit_json() -> anyhow::Result<()> {
    test_cli_edit_impl("json").await
}

#[tokio::test]
async fn test_cli_edit_libsql() -> anyhow::Result<()> {
    test_cli_edit_impl("libsql").await
}

async fn test_cli_edit_merge_impl(storage_type: &str) -> anyhow::Result<()> {
    let env = CliTestEnv::new(storage_type)?;
    let storage: Box<dyn Storage + Send + Sync> = if storage_type == "json" {
        Box::new(JsonStorage::new(Some(env.storage_path.to_path_buf()))?)
    } else {
        Box::new(LibSQLStorage::new(Some(env.storage_path.to_path_buf())).await?)
    };

    let mut prompt = Prompt::new(
        "A prompt to edit with merge",
        Some(vec!["tag1".to_string(), "tag2".to_string()]),
        Some(vec!["cat1".to_string(), "cat2".to_string()]),
    );
    storage.save_prompt(&mut prompt).await?;
    let _old_hash = prompt.hash.clone();

    // Add a tag
    let mut cmd = Command::cargo_bin(r#"prompts-cli"#)?;
    cmd.arg("--config")
        .arg(&env.config_path)
        .arg("edit")
        .arg("prompt to edit with merge")
        .arg("--add-tags")
        .arg("tag3");
    cmd.assert().success();

    // Remove a tag
    let mut cmd = Command::cargo_bin(r#"prompts-cli"#)?;
    cmd.arg("--config")
        .arg(&env.config_path)
        .arg("edit")
        .arg("prompt to edit with merge")
        .arg("--remove-tags")
        .arg("tag1");

    cmd.assert().success();

    // Add a category
    let mut cmd = Command::cargo_bin(r#"prompts-cli"#)?;
    cmd.arg("--config")
        .arg(&env.config_path)
        .arg("edit")
        .arg("prompt to edit with merge")
        .arg("--add-categories")
        .arg("cat3");

    cmd.assert().success();

    // Remove a category
    let mut cmd = Command::cargo_bin(r#"prompts-cli"#)?;
    cmd.arg("--config")
        .arg(&env.config_path)
        .arg("edit")
        .arg("prompt to edit with merge")
        .arg("--remove-categories")
        .arg("cat1");

    cmd.assert().success();

    let prompts = storage.load_prompts().await?;
    let edited_prompt = prompts.iter().find(|p| p.content == "A prompt to edit with merge").unwrap();

    assert_eq!(
        edited_prompt.tags,
        Some(vec!["tag2".to_string(), "tag3".to_string()])
    );
    assert_eq!(
        edited_prompt.categories,
        Some(vec!["cat2".to_string(), "cat3".to_string()])
    );

    Ok(())
}

#[tokio::test]
async fn test_cli_edit_merge_json() -> anyhow::Result<()> {
    test_cli_edit_merge_impl("json").await
}

#[tokio::test]
async fn test_cli_edit_merge_libsql() -> anyhow::Result<()> {
    test_cli_edit_merge_impl("libsql").await
}

async fn test_cli_edit_add_tags_impl(storage_type: &str) -> anyhow::Result<()> {
    let env = CliTestEnv::new(storage_type)?;
    let storage: Box<dyn Storage + Send + Sync> = if storage_type == "json" {
        Box::new(JsonStorage::new(Some(env.storage_path.to_path_buf()))?)
    } else {
        Box::new(LibSQLStorage::new(Some(env.storage_path.to_path_buf())).await?)
    };

    let mut prompt = Prompt::new(
        "A prompt to edit with merge",
        Some(vec!["tag1".to_string()]),
        None,
    );
    storage.save_prompt(&mut prompt).await?;
    let old_hash = prompt.hash.clone();

    // Add a tag
    let mut cmd = Command::cargo_bin(r#"prompts-cli"#)?;
    cmd.arg("--config")
        .arg(&env.config_path)
        .arg("edit")
        .arg("prompt to edit with merge")
        .arg("--add-tags")
        .arg("tag2");
    cmd.assert().success();

    let prompts = storage.load_prompts().await?;
    let edited_prompt = prompts.iter().find(|p| p.content == "A prompt to edit with merge").unwrap();

    let mut expected_tags = vec!["tag1".to_string(), "tag2".to_string()];
    expected_tags.sort();
    let mut actual_tags = edited_prompt.tags.clone().unwrap();
    actual_tags.sort();

    assert_eq!(
        actual_tags,
        expected_tags,
        "Tags should be merged correctly"
    );

    Ok(())
}

#[tokio::test]
async fn test_cli_edit_add_tags_json() -> anyhow::Result<()> {
    test_cli_edit_add_tags_impl("json").await
}

#[tokio::test]
async fn test_cli_edit_add_tags_libsql() -> anyhow::Result<()> {
    test_cli_edit_add_tags_impl("libsql").await
}

async fn test_cli_edit_remove_tags_impl(storage_type: &str) -> anyhow::Result<()> {
    let env = CliTestEnv::new(storage_type)?;
    let storage: Box<dyn Storage + Send + Sync> = if storage_type == "json" {
        Box::new(JsonStorage::new(Some(env.storage_path.to_path_buf()))?)
    } else {
        Box::new(LibSQLStorage::new(Some(env.storage_path.to_path_buf())).await?)
    };

    let mut prompt = Prompt::new(
        "A prompt to remove tags from",
        Some(vec!["tag1".to_string(), "tag2".to_string()]),
        None,
    );
    storage.save_prompt(&mut prompt).await?;

    // Remove a tag
    let mut cmd = Command::cargo_bin(r#"prompts-cli"#)?;
    cmd.arg("--config")
        .arg(&env.config_path)
        .arg("edit")
        .arg("A prompt to remove tags from")
        .arg("--remove-tags")
        .arg("tag1");
    cmd.assert().success();

    let prompts = storage.load_prompts().await?;
    let edited_prompt = prompts.iter().find(|p| p.content == "A prompt to remove tags from").unwrap();

    let mut expected_tags = vec!["tag2".to_string()];
    expected_tags.sort();
    let mut actual_tags = edited_prompt.tags.clone().unwrap();
    actual_tags.sort();

    assert_eq!(
        actual_tags,
        expected_tags,
        "Tags should be removed correctly"
    );

    Ok(())
}

#[tokio::test]
async fn test_cli_edit_remove_tags_json() -> anyhow::Result<()> {
    test_cli_edit_remove_tags_impl("json").await
}

#[tokio::test]
async fn test_cli_edit_remove_tags_libsql() -> anyhow::Result<()> {
    test_cli_edit_remove_tags_impl("libsql").await
}

async fn test_cli_edit_add_categories_impl(storage_type: &str) -> anyhow::Result<()> {
    let env = CliTestEnv::new(storage_type)?;
    let storage: Box<dyn Storage + Send + Sync> = if storage_type == "json" {
        Box::new(JsonStorage::new(Some(env.storage_path.to_path_buf()))?)
    } else {
        Box::new(LibSQLStorage::new(Some(env.storage_path.to_path_buf())).await?)
    };

    let mut prompt = Prompt::new(
        "A prompt to add categories to",
        None,
        Some(vec!["cat1".to_string()]),
    );
    storage.save_prompt(&mut prompt).await?;

    // Add a category
    let mut cmd = Command::cargo_bin(r#"prompts-cli"#)?;
    cmd.arg("--config")
        .arg(&env.config_path)
        .arg("edit")
        .arg("A prompt to add categories to")
        .arg("--add-categories")
        .arg("cat2");
    cmd.assert().success();

    let prompts = storage.load_prompts().await?;
    let edited_prompt = prompts.iter().find(|p| p.content == "A prompt to add categories to").unwrap();

    let mut expected_categories = vec!["cat1".to_string(), "cat2".to_string()];
    expected_categories.sort();
    let mut actual_categories = edited_prompt.categories.clone().unwrap();
    actual_categories.sort();

    assert_eq!(
        actual_categories,
        expected_categories,
        "Categories should be added correctly"
    );

    Ok(())
}

#[tokio::test]
async fn test_cli_edit_add_categories_json() -> anyhow::Result<()> {
    test_cli_edit_add_categories_impl("json").await
}

#[tokio::test]
async fn test_cli_edit_add_categories_libsql() -> anyhow::Result<()> {
    test_cli_edit_add_categories_impl("libsql").await
}

async fn test_cli_edit_remove_categories_impl(storage_type: &str) -> anyhow::Result<()> {
    let env = CliTestEnv::new(storage_type)?;
    let storage: Box<dyn Storage + Send + Sync> = if storage_type == "json" {
        Box::new(JsonStorage::new(Some(env.storage_path.to_path_buf()))?)
    } else {
        Box::new(LibSQLStorage::new(Some(env.storage_path.to_path_buf())).await?)
    };

    let mut prompt = Prompt::new(
        "A prompt to remove categories from",
        None,
        Some(vec!["cat1".to_string(), "cat2".to_string()]),
    );
    storage.save_prompt(&mut prompt).await?;

    // Remove a category
    let mut cmd = Command::cargo_bin(r#"prompts-cli"#)?;
    cmd.arg("--config")
        .arg(&env.config_path)
        .arg("edit")
        .arg("A prompt to remove categories from")
        .arg("--remove-categories")
        .arg("cat1");
    cmd.assert().success();

    let prompts = storage.load_prompts().await?;
    let edited_prompt = prompts.iter().find(|p| p.content == "A prompt to remove categories from").unwrap();

    let mut expected_categories = vec!["cat2".to_string()];
    expected_categories.sort();
    let mut actual_categories = edited_prompt.categories.clone().unwrap();
    actual_categories.sort();

    assert_eq!(
        actual_categories,
        expected_categories,
        "Categories should be removed correctly"
    );

    Ok(())
}

#[tokio::test]
async fn test_cli_edit_remove_categories_json() -> anyhow::Result<()> {
    test_cli_edit_remove_categories_impl("json").await
}

#[tokio::test]
async fn test_cli_edit_remove_categories_libsql() -> anyhow::Result<()> {
    test_cli_edit_remove_categories_impl("libsql").await
}

async fn test_cli_show_with_tag_filter_impl(storage_type: &str) -> anyhow::Result<()> {
    let env = CliTestEnv::new(storage_type)?;
    let storage: Box<dyn Storage + Send + Sync> = if storage_type == "json" {
        Box::new(JsonStorage::new(Some(env.storage_path.to_path_buf()))?)
    } else {
        Box::new(LibSQLStorage::new(Some(env.storage_path.to_path_buf())).await?)
    };

    let mut prompt1 = Prompt::new("A prompt with tag", Some(vec!["test-tag".to_string()]), None);
    storage.save_prompt(&mut prompt1).await?;

    let mut prompt2 = Prompt::new("A prompt without tag", None, None);
    storage.save_prompt(&mut prompt2).await?;

    let mut cmd = Command::cargo_bin(r#"prompts-cli"#)?;
    cmd.arg("--config")
        .arg(&env.config_path)
        .arg("show")
        .arg("prompt")
        .arg("--tags")
        .arg("test-tag");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A prompt with tag"))
        .stdout(predicate::str::contains("A prompt without tag").not());

    Ok(())
}

#[tokio::test]
async fn test_cli_show_with_tag_filter_json() -> anyhow::Result<()> {
    test_cli_show_with_tag_filter_impl("json").await
}

#[tokio::test]
async fn test_cli_show_with_tag_filter_libsql() -> anyhow::Result<()> {
    test_cli_show_with_tag_filter_impl("libsql").await
}
