use std::fs::File;
use std::io::Write;
use std::process::Command;
use tempfile::tempdir;
use std::fs;

#[test]
fn test_cli_list() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("prompts.json");

    let mut file = File::create(&file_path).unwrap();
    file.write_all(b"[\n  {\n    \"name\": \"Test Prompt\",\n    \"text\": \"This is a test prompt.\",\n    \"tags\": [],\n    \"categories\": []\n  }\n]").unwrap();

    let output = Command::new("cargo")
        .args(["run", "--file", file_path.to_str().unwrap(), "--", "list"])
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    assert!(stdout.contains("Test Prompt: This is a test prompt."));
}

#[test]
fn test_cli_show() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("prompts.json");

    let mut file = File::create(&file_path).unwrap();
    file.write_all(b"[\n  {\n    \"name\": \"Test Prompt\",\n    \"text\": \"This is a test prompt.\",\n    \"tags\": [],\n    \"categories\": []\n  }\n]").unwrap();

    let output = Command::new("cargo")
        .args(["run", "--file", file_path.to_str().unwrap(), "--", "show", "Test Prompt"])
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    assert_eq!(stdout.trim(), "This is a test prompt.");
}

#[test]
fn test_cli_show_not_found() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("prompts.json");

    let mut file = File::create(&file_path).unwrap();
    file.write_all(b"[]").unwrap();

    let output = Command::new("cargo")
        .args(["run", "--file", file_path.to_str().unwrap(), "--", "show", "Non-existent Prompt"])
        .output()
        .expect("failed to execute process");

    let stderr = String::from_utf8(output.stderr).unwrap();

    assert!(!output.status.success());
    assert!(stderr.contains("Prompt 'Non-existent Prompt' not found"));
}

#[test]
fn test_cli_generate() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("prompts.json");

    let mut file = File::create(&file_path).unwrap();
    file.write_all(b"[\n  {\n    \"name\": \"Test Prompt\",\n    \"text\": \"This is a test prompt.\",\n    \"tags\": [],\n    \"categories\": []\n  }\n]").unwrap();

    let output = Command::new("cargo")
        .args(["run", "--file", file_path.to_str().unwrap(), "--", "generate", "Test Prompt", "--generator", "mock"])
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    if !output.status.success() {
        eprintln!("Command failed with stdout:\n{}", stdout);
        eprintln!("Command failed with stderr:\n{}", stderr);
    }

    assert!(output.status.success());
    assert!(stdout.contains("Generated text for 'This is a test prompt.': This is a test prompt.\n(This is a mock generation)"));
}

#[test]
fn test_cli_add() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("prompts.json");

    // Ensure the file is empty or doesn't exist initially
    let _ = fs::remove_file(&file_path);

    let output = Command::new("cargo")
        .args([
            "run",
            "--file",
            file_path.to_str().unwrap(),
            "--",
            "add",
            "New Prompt",
            "This is a new prompt.",
            "--tags",
            "tag1,tag2",
            "--categories",
            "cat1,cat2",
        ])
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    if !output.status.success() {
        eprintln!("Command failed with stdout:\n{}", stdout);
        eprintln!("Command failed with stderr:\n{}", stderr);
    }

    assert!(output.status.success());
    assert!(stdout.contains("Prompt 'New Prompt' added successfully."));

    // Verify the content of the file
    let prompts_content = fs::read_to_string(&file_path).unwrap();
    let prompts: serde_json::Value = serde_json::from_str(&prompts_content).unwrap();

    assert_eq!(prompts.as_array().unwrap().len(), 1);
    let new_prompt = &prompts.as_array().unwrap()[0];
    assert_eq!(new_prompt["name"], "New Prompt");
    assert_eq!(new_prompt["text"], "This is a new prompt.");
    assert_eq!(new_prompt["tags"].as_array().unwrap().len(), 2);
    assert!(new_prompt["tags"].as_array().unwrap().contains(&serde_json::Value::String("tag1".to_string())));
    assert!(new_prompt["tags"].as_array().unwrap().contains(&serde_json::Value::String("tag2".to_string())));
    assert_eq!(new_prompt["categories"].as_array().unwrap().len(), 2);
    assert!(new_prompt["categories"].as_array().unwrap().contains(&serde_json::Value::String("cat1".to_string())));
    assert!(new_prompt["categories"].as_array().unwrap().contains(&serde_json::Value::String("cat2".to_string())));
}

#[test]
fn test_cli_search() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("prompts.json");

    let mut file = File::create(&file_path).unwrap();
    file.write_all(b"[\n  {\n    \"name\": \"Prompt One\",\n    \"text\": \"This is the first prompt.\",\n    \"tags\": [\"tagA\", \"tagB\"],\n    \"categories\": [\"catX\"]\n  },\n  {\n    \"name\": \"Prompt Two\",\n    \"text\": \"Second prompt here.\",\n    \"tags\": [\"tagB\", \"tagC\"],\n    \"categories\": [\"catY\"]\n  },\n  {\n    \"name\": \"Another Prompt\",\n    \"text\": \"A third one for testing.\",\n    \"tags\": [\"tagA\"],\n    \"categories\": [\"catX\", \"catZ\"]\n  }\n]").unwrap();

    // Test search by query
    let output = Command::new("cargo")
        .args(["run", "--file", file_path.to_str().unwrap(), "--", "search", "--query", "first"])
        .output()
        .expect("failed to execute process");
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    if !output.status.success() {
        eprintln!("Command failed with stdout:\n{}", stdout);
        eprintln!("Command failed with stderr:\n{}", stderr);
    }
    assert!(output.status.success());
    assert!(stdout.contains("Name: Prompt One"));
    assert!(!stdout.contains("Name: Prompt Two"));

    // Test search by tag
    let output = Command::new("cargo")
        .args(["run", "--file", file_path.to_str().unwrap(), "--", "search", "--query", "", "--tags", "tagC"])
        .output()
        .expect("failed to execute process");
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    if !output.status.success() {
        eprintln!("Command failed with stdout:\n{}", stdout);
        eprintln!("Command failed with stderr:\n{}", stderr);
    }
    assert!(output.status.success());
    assert!(stdout.contains("Name: Prompt Two"));
    assert!(!stdout.contains("Name: Prompt One"));

    // Test search by category and query
    let output = Command::new("cargo")
        .args(["run", "--file", file_path.to_str().unwrap(), "--", "search", "--query", "third", "--categories", "catX"])
        .output()
        .expect("failed to execute process");
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    if !output.status.success() {
        eprintln!("Command failed with stdout:\n{}", stdout);
        eprintln!("Command failed with stderr:\n{}", stderr);
    }
    assert!(output.status.success());
    assert!(stdout.contains("Name: Another Prompt"));
    assert!(!stdout.contains("Name: Prompt One"));
}