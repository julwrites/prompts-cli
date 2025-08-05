# TODO for prompts-cli v2

This document outlines the next steps for the `prompts-cli` project, focusing on addressing the gaps identified in `EVAL.md`. The immediate priority is to fully implement the first three user stories.

---

## **P0: Core Functionality and Configuration**

### **US-001: Automatic and Correct Storage Location**

- **Task:** Fix configuration file loading.
    - **Sub-task:** Modify `main.rs` to look for `config.toml` in the user's default configuration directory (e.g., `~/.config/prompts-cli/config.toml`) by default.
    - **Sub-task:** Ensure that the `--config` flag still allows overriding the default location.
    - **Test:** Create an integration test that confirms the CLI loads configuration from `~/.config/prompts-cli/config.toml` when no `--config` flag is provided.
    - **Test:** Verify that the test for the `--config` flag (`test_cli_config_file`) still passes.

### **US-002: Graceful Handling of Duplicate Prompts**

- **Task:** Implement true de-duplication for the `add` command.
    - **Sub-task:** In `prompts_cli::core::Prompts::add_prompt`, add a check to see if a prompt with the same hash already exists in the storage.
    - **Sub-task:** If the prompt already exists, the function should do nothing and return `Ok(())`, making the operation a true "no-op".
    - **Sub-task:** The CLI should provide feedback to the user that the prompt already exists.
    - **Test:** Write a unit test for `Prompts::add_prompt` that asserts no save operation is performed if the prompt hash already exists.
    - **Test:** Write an integration test for `prompts-cli add` that verifies a duplicate prompt is not added and the user is notified.

### **US-003: Fuzzy Search (Completed)**

- **Task:** This user story is fully implemented. No new tasks are required.

---
