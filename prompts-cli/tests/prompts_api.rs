use prompts_cli::{
    Prompt,
    storage::JsonStorage,
    Prompts
};
use tempfile::tempdir;

#[tokio::test]
async fn test_prompts_api() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let storage_path = dir.path().to_path_buf();
    let storage = JsonStorage::new(Some(storage_path.clone()))?;
    let prompts_api = Prompts::new(Box::new(storage));

    // Test adding a prompt
    let mut prompt = Prompt::new("test content", Some(vec!["tag1".to_string()]), None);
    let added = prompts_api.add_prompt(&mut prompt).await?;
    assert!(added);
    assert!(!prompt.hash.is_empty());

    // Test listing prompts
    let listed_prompts = prompts_api.list_prompts(None).await?;
    assert_eq!(listed_prompts.len(), 1);
    assert_eq!(listed_prompts[0].content, "test content");

    // Test showing a prompt
    let shown_prompts = prompts_api.show_prompt("test", None).await?;
    assert_eq!(shown_prompts.len(), 1);
    assert_eq!(shown_prompts[0].content, "test content");

    // Test editing a prompt
    prompts_api.edit_prompt(
        &prompt.hash,
        Some("edited content".to_string()),
        Some(vec!["tag2".to_string()]),
        None,
        None,
        None,
    ).await?;
    let updated_prompts = prompts_api.list_prompts(None).await?;
    assert_eq!(updated_prompts.len(), 1);
    assert_eq!(updated_prompts[0].content, "edited content");
    // The new implementation of edit_prompt merges tags, so the old tag is still there.
    let mut expected_tags = vec!["tag1".to_string(), "tag2".to_string()];
    expected_tags.sort();
    let mut actual_tags = updated_prompts[0].tags.clone().unwrap();
    actual_tags.sort();
    assert_eq!(actual_tags, expected_tags);

    // Test deleting a prompt
    prompts_api.delete_prompt(&updated_prompts[0].hash).await?;
    let remaining_prompts = prompts_api.list_prompts(None).await?;
    assert!(remaining_prompts.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_add_duplicate_prompt() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let storage_path = dir.path().to_path_buf();
    let storage = JsonStorage::new(Some(storage_path.clone()))?;
    let prompts_api = Prompts::new(Box::new(storage));

    let mut prompt = Prompt::new("duplicate content", None, None);

    // First add should succeed
    let added = prompts_api.add_prompt(&mut prompt).await?;
    assert!(added);

    // Second add should not add a new prompt and return false
    let added_again = prompts_api.add_prompt(&mut prompt).await?;
    assert!(!added_again);

    // Verify that only one prompt exists
    let listed_prompts = prompts_api.list_prompts(None).await?;
    assert_eq!(listed_prompts.len(), 1);

    Ok(())
}
