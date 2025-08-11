# TODO for prompts-cli v2

This document outlines the next steps for the `prompts-cli` project, focusing on addressing the gaps identified in `EVAL.md`. The immediate priority is to fully implement the first three user stories.

---

## **P0: Core Functionality and Configuration**

### **US-001: Automatic and Correct Storage Location (Completed)**

- **Task:** Fix configuration file loading.
    - **Sub-task:** Modify `main.rs` to look for `config.toml` in the user's default configuration directory (e.g., `~/.config/prompts-cli/config.toml`) by default. (Done)
    - **Sub-task:** Ensure that the `--config` flag still allows overriding the default location. (Done)
    - **Test:** Create an integration test that confirms the CLI loads configuration from `~/.config/prompts-cli/config.toml` when no `--config` flag is provided. (Done)
    - **Test:** Verify that the test for the `--config` flag (`test_cli_config_file`) still passes. (Done)

### **US-002: Graceful Handling of Duplicate Prompts (Completed)**

- **Task:** Implement true de-duplication for the `add` command.
    - **Sub-task:** In `prompts_cli::core::Prompts::add_prompt`, add a check to see if a prompt with the same hash already exists in the storage. (Done)
    - **Sub-task:** If the prompt already exists, the function should do nothing and return `Ok(())`, making the operation a true "no-op". (Done)
    - **Sub-task:** The CLI should provide feedback to the user that the prompt already exists. (Done)
    - **Test:** Write a unit test for `Prompts::add_prompt` that asserts no save operation is performed if the prompt hash already exists. (Done)
    - **Test:** Write an integration test for `prompts-cli add` that verifies a duplicate prompt is not added and the user is notified. (Done)

### **US-003: Fuzzy Search (Completed)**

- **Task:** This user story is fully implemented. No new tasks are required.

---

## **P1: Feature Completeness**

### **US-009: Complete the Tagging Feature**

-   **Task:** Add tag-based filtering to the `list` command. (Done)
    -   **Sub-task:** Add a `--tag` option to the `list` command in `main.rs`. (Done)
    -   **Sub-task:** Update `Prompts::list_prompts` to accept and pass tag filters to `search_prompts`. (Done)
-   **Task:** Ensure all search-based commands can filter by tag.
    -   **Sub-task:** Modify `Prompts::show_prompt` and other relevant functions to accept and use tag/category filters. (Done)

### **Improve `edit` Command Ergonomics (Completed)**

-   **Task:** Refactor the `edit` command to merge changes instead of requiring full re-specification. (Done)
    -   **Sub-task:** Fetch the existing prompt's metadata (tags, categories). (Done)
    -   **Sub-task:** Allow users to add or remove specific tags without re-entering the entire list. (Done)

---

## **P2: Architectural Improvements**

### **Implement Structured Error Handling**

-   **Task:** Add a global `--output json` flag for structured error output.
    -   **Sub-task:** Create a serializable `Error` struct.
    -   **Sub-task:** In `main.rs`, catch `Err` results and, if the flag is present, print the serialized JSON error object.

### **Enhance Test Coverage**

-   **Task:** Add integration tests for failure cases identified in `EVAL.md`.
    -   **Test:** Create a test case for adding a duplicate prompt.
    -   **Test:** Create a test case for loading configuration from the correct default path.
    -   **Test:** Add tests for the new features as they are developed.
