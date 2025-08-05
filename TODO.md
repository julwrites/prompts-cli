# LibSQL Storage Implementation Plan (TDD)

This plan outlines the steps to migrate from the current file-based storage to a LibSQL-based storage system, following Test-Driven Development (TDD) principles.

- **Task 1: Add LibSQL dependencies** - **COMPLETED**
    - Sub-task: Add the `libsql` crate to `prompts-cli/Cargo.toml`. Consider adding `tokio` if an async runtime is required for the `libsql` crate.
    - Test: The build should pass, and the new dependencies should be available.

- **Task 2: Create `LibSQLStorage` struct and `new` function** - **COMPLETED**
    - Sub-task: In `prompts-cli/src/storage.rs`, define a new struct `LibSQLStorage`.
    - Sub-task: Implement a `new` function for `LibSQLStorage` that takes a database path. This function will establish a connection to the LibSQL database.
    - Sub-task: The `new` function should also handle database schema creation (e.g., creating a `prompts` table).
    - Test: Write a test in `prompts-cli/tests/storage.rs` that calls `LibSQLStorage::new` and asserts that the database file is created and the `prompts` table exists.

- **Task 3: Implement `save_prompt`** - **COMPLETED**
    - Test: In `prompts-cli/tests/storage.rs`, write a failing test for `save_prompt` on `LibSQLStorage`. The test should attempt to save a `Prompt` and then verify its existence directly in the database.
    - Sub-task: Implement the `save_prompt` method for `LibSQLStorage` in `prompts-cli/src/storage.rs`. This method will insert a new prompt record into the `prompts` table.
    - Sub-task: Ensure the test passes after implementation.

- **Task 4: Implement `load_prompts`** - **COMPLETED**
    - Test: In `prompts-cli/tests/storage.rs`, write a failing test for `load_prompts` on `LibSQLStorage`. The test should save a known set of prompts and then fail to load them.
    - Sub-task: Implement the `load_prompts` method for `LibSQLStorage`. This method will query the `prompts` table and return a `Vec<Prompt>`.
    - Sub-task: Ensure the test passes after implementation.

- **Task 5: Implement `delete_prompt`** - **COMPLETED**
    - Test: In `prompts-cli/tests/storage.rs`, write a failing test for `delete_prompt` on `LibSQLStorage`. The test should save a prompt, attempt to delete it, and then assert it's no longer in the database.
    - Sub-task: Implement the `delete_prompt` method for `LibSQLStorage`. This method will delete a prompt from the `prompts` table based on its hash.
    - Sub-task: Ensure the test passes after implementation.

- **Task 6: Integrate `LibSQLStorage` into the application**
    - Sub-task: Modify the application's entry point (likely in `prompts-cli/src/main.rs` and `prompts-cli/src/lib.rs`) to use `LibSQLStorage` instead of `JsonStorage`.
    - Sub-task: This may involve adding a configuration option to `config.rs` to allow the user to select the storage backend and specify the database path. For now, a direct replacement is sufficient to prove the concept.
    - Test: Manually run the CLI to ensure all commands (`add`, `list`, `show`, `edit`, `delete`) work correctly with the new `LibSQLStorage` backend. Existing integration tests should be adapted and should all pass.

- **Task 7: Refactor and clean up**
    - Sub-task: Remove the `JsonStorage` implementation if it's no longer needed, or keep it as an alternative backend.
    - Sub-task: Review the code for any necessary refactoring and add documentation for the new storage implementation.

---

# TODO for `prompts-cli`

This document outlines the implementation plan for the `prompts-cli` project, based on the requirements in the PRD.

## Current Task Status Summary

All tasks up to and including Phase 3 (Core Library and Storage, CLI, Configuration and Import/Export) are **COMPLETED**.

## Phase 1: Core Library and Storage

- **Task 1: Define the `Prompt` struct**
    - Sub-task: Define the fields for the `Prompt` struct, including content, tags, and categories.
    - Test: Write a unit test to ensure that the `Prompt` struct can be created and that its fields can be accessed.
    - **COMPLETED**

- **Task 2: Implement the storage layer**
    - Sub-task: Create a `Storage` trait that defines the interface for the storage layer.
    - Sub-task: Implement a `JsonStorage` struct that implements the `Storage` trait and stores prompts as individual JSON files.
    - Sub-task: Implement content-addressable storage by using the SHA256 hash of the prompt content as the filename.
    - Test: Write unit tests for the `JsonStorage` struct to ensure that it can add, retrieve, and delete prompts.
    - **COMPLETED**

- **Task 3: Implement the core library API**
    - Sub-task: Create a `Prompts` struct that encapsulates the core logic for prompt management.
    - Sub-task: Implement the `add`, `list`, `show`, `edit`, and `delete` functions in the `Prompts` struct.
    - Test: Write unit tests for the `Prompts` struct to ensure that it is working correctly.
    - **COMPLETED**

## Phase 2: CLI

- **Task 4: Implement the CLI commands**
    - Sub-task: Use the `clap` crate to define the CLI commands.
    - Sub-task: Implement the `add` command.
        - Test: `test_cli_add` - Verify prompt is added and file exists.
    - Sub-task: Implement the `list` command.
        - Test: `test_cli_list` - Verify all prompts are listed.
    - Sub-task: Implement the `show` command.
        - Test: `test_cli_show` - Verify a single prompt is shown.
        - Test: `test_cli_show_multiple` - Verify multiple prompts are returned for ambiguous queries.
    - Sub-task: Implement the `edit` command.
        - Test: `test_cli_edit` - Verify a prompt is updated and old file is removed.
    - Sub-task: Implement the `delete` command.
        - Test: `test_cli_delete` - Verify a prompt is deleted and file is removed.
    - Sub-task: Implement the `--tag` flag for the `add`, `edit`, and `list` commands.
    - **COMPLETED**

- **Task 5: Implement fuzzy search**
    - Sub-task: Use the `fuzzy-matcher` crate to implement fuzzy search for the `show`, `edit`, and `delete` commands.
    - Test: Write a unit test for the fuzzy search logic to ensure that it is working correctly.
    - **COMPLETED**

- **Task 6: Implement templating**
    - Sub-task: Use the `tera` crate to implement templating for the `generate` command.
    - Sub-task: Implement the `--variable` flag for the `generate` command.
    - Test: Write a unit test for the templating logic to ensure that it is working correctly.
    - **COMPLETED**

## Phase 3: Configuration and Import/Export

- **Task 7: Implement the configuration file**
    - Sub-task: Use the `toml` crate to parse the configuration file.
    - Sub-task: Implement the `--config` flag to allow users to specify a custom configuration file.
    - Test: `test_cli_config_file` - Verify CLI uses specified config file for storage path.
    - **COMPLETED**

- **Task 8: Implement import/export**
    - Sub-task: Implement the `import` command.
        - Test: `test_cli_import_export` - Verify prompts are imported from a directory.
    - Sub-task: Implement the `export` command.
        - Test: `test_cli_import_export` - Verify prompts are exported to a directory.
    - **COMPLETED**

## Refactoring/Maintenance

- **Task 9: Refactor `tests/cli.rs`**
    - Status: **COMPLETED**