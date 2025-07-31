use prompts_cli::{Prompt, search_prompts};

#[test]
fn test_fuzzy_search_prompts() {
    let prompts = vec![
        Prompt {
            hash: "1".to_string(),
            content: "This is the first prompt.".to_string(),
            tags: Some(vec!["tagA".to_string(), "tagB".to_string()]),
            categories: Some(vec!["catX".to_string()]),
        },
        Prompt {
            hash: "2".to_string(),
            content: "Second prompt here.".to_string(),
            tags: Some(vec!["tagB".to_string(), "tagC".to_string()]),
            categories: Some(vec!["catY".to_string()]),
        },
        Prompt {
            hash: "3".to_string(),
            content: "A third one for testing.".to_string(),
            tags: Some(vec!["tagA".to_string()]),
            categories: Some(vec!["catX".to_string(), "catZ".to_string()]),
        },
    ];

    // Test fuzzy search by query
    let results = search_prompts(&prompts, "frst prmpt", &[], &[]);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].content, "This is the first prompt.");

    // Test fuzzy search with no good match
    let results = search_prompts(&prompts, "xyz", &[], &[]);
    assert_eq!(results.len(), 0);

    // Test search by tag
    let results = search_prompts(&prompts, "", &["tagC".to_string()], &[]);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].content, "Second prompt here.");

    // Test search by category and fuzzy query
    let results = search_prompts(&prompts, "thrd tst", &[], &["catX".to_string()]);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].content, "A third one for testing.");

    // Test filtering that results in no matches
    let results = search_prompts(&prompts, "", &["tagD".to_string()], &[]);
    assert_eq!(results.len(), 0);
}

