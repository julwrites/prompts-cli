use prompts_cli::{load_prompts, search_prompts, Prompt};
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_search_prompts() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, r#"[
  {{
    "name": "Prompt One",
    "text": "This is the first prompt.",
    "tags": ["tagA", "tagB"],
    "categories": ["catX"]
  }},
  {{
    "name": "Prompt Two",
    "text": "Second prompt here.",
    "tags": ["tagB", "tagC"],
    "categories": ["catY"]
  }},
  {{
    "name": "Another Prompt",
    "text": "A third one for testing.",
    "tags": ["tagA"],
    "categories": ["catX", "catZ"]
  }}
]"#)?;

    let prompts = load_prompts(file.path().to_str().unwrap())?;

    // Test search by query
    let results = search_prompts(&prompts, "first", &[], &[]);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Prompt One");

    // Test search by tag
    let results = search_prompts(&prompts, "", &["tagC".to_string()], &[]);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Prompt Two");

    // Test search by category and query
    let results = search_prompts(&prompts, "third", &[], &["catX".to_string()]);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Another Prompt");

    Ok(())
}
