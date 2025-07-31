use prompts_cli::Prompt;

#[test]
fn test_create_prompt() {
    let prompt = Prompt::new("test content", None, None);
    assert_eq!(prompt.content, "test content");
    assert_eq!(prompt.tags, None);
    assert_eq!(prompt.categories, None);
}
