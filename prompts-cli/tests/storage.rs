use prompts_cli::{
    Prompt,
    storage::{Storage, JsonStorage}
};
use tempfile::tempdir;

#[test]
fn test_json_storage() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let storage_path = dir.path().to_path_buf();
    let storage = JsonStorage::new(Some(storage_path.clone()))?;

    // Test saving a prompt
    let mut prompt = Prompt::new("test content", Some(vec!["tag1".to_string()]), None);
    storage.save_prompt(&mut prompt)?;
    assert!(!prompt.hash.is_empty());
    let prompt_file = storage_path.join(format!("{}.json", prompt.hash));
    assert!(prompt_file.exists());

    // Test loading prompts
    let loaded_prompts = storage.load_prompts()?;
    assert_eq!(loaded_prompts.len(), 1);
    assert_eq!(loaded_prompts[0].content, "test content");
    assert_eq!(loaded_prompts[0].tags, Some(vec!["tag1".to_string()]));

    // Test deleting a prompt
    storage.delete_prompt(&prompt.hash)?;
    assert!(!prompt_file.exists());
    let loaded_prompts_after_delete = storage.load_prompts()?;
    assert!(loaded_prompts_after_delete.is_empty());

    Ok(())
}
