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