use prompts_cli::{search_prompts, Prompt};

#[test]
fn test_fuzzy_search_prompts() {
    let prompts = vec![
        Prompt {
            hash: "1".to_string(),
            text: "This is the first prompt.".to_string(),
            tags: vec!["tagA".to_string(), "tagB".to_string()],
            categories: vec!["catX".to_string()],
        },
        Prompt {
            hash: "2".to_string(),
            text: "Second prompt here.".to_string(),
            tags: vec!["tagB".to_string(), "tagC".to_string()],
            categories: vec!["catY".to_string()],
        },
        Prompt {
            hash: "3".to_string(),
            text: "A third one for testing.".to_string(),
            tags: vec!["tagA".to_string()],
            categories: vec!["catX".to_string(), "catZ".to_string()],
        },
    ];

    // Test fuzzy search by query
    let results = search_prompts(&prompts, "frst prmpt", &[], &[]);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].text, "This is the first prompt.");

    // Test fuzzy search with no good match
    let results = search_prompts(&prompts, "xyz", &[], &[]);
    assert_eq!(results.len(), 0);

    // Test search by tag
    let results = search_prompts(&prompts, "", &["tagC".to_string()], &[]);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].text, "Second prompt here.");

    // Test search by category and fuzzy query
    let results = search_prompts(&prompts, "thrd tst", &[], &["catX".to_string()]);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].text, "A third one for testing.");

    // Test filtering that results in no matches
    let results = search_prompts(&prompts, "", &["tagD".to_string()], &[]);
    assert_eq!(results.len(), 0);
}
