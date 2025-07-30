# TODO: Prompts CLI and Multi-Frontend Prompt Management Solution

This document outlines the tasks required to complete the features specified in the PRD. The tasks are broken down into a TDD (Test-Driven Development) approach.

## Phase 1: Core CLI Features (TDD)

### Task 1: Implement `edit` command

*   **Subtask 1.1: Write a failing test for the `edit` command.**
    *   Create a test that tries to edit a prompt's text, tags, and categories.
    *   Assert that the prompt has been updated correctly in the JSON file.
*   **Subtask 1.2: Implement the `edit` command in `src/main.rs`.**
    *   Add an `Edit` variant to the `Commands` enum.
    *   Implement the logic to find the prompt by name, update its fields, and save the changes.
*   **Subtask 1.3: Run the test and ensure it passes.**

### Task 2: Implement `delete` command

*   **Subtask 2.1: Write a failing test for the `delete` command.**
    *   Create a test that adds a prompt, deletes it, and then asserts that the prompt is no longer in the JSON file.
*   **Subtask 2.2: Implement the `delete` command in `src/main.rs`.**
    *   Add a `Delete` variant to the `Commands` enum.
    *   Implement the logic to find the prompt by name and remove it from the list.
*   **Subtask 2.3: Run the test and ensure it passes.**

### Task 3: Refactor `search` functionality

*   **Subtask 3.1: Write a failing test for the `search_prompts` function in `prompts-core`.**
    *   Create a test in `prompts-core` that covers various search scenarios (by query, tags, categories).
*   **Subtask 3.2: Move the `search_prompts` function from `src/main.rs` to `prompts-core/src/lib.rs`.**
    *   Update `src/main.rs` to call the new function from the `prompts-core` crate.
*   **Subtask 3.3: Run the tests for both `prompts-cli` and `prompts-core` and ensure they pass.**

## Phase 2: TUI Development

### Task 4: Integrate the TUI

*   **Subtask 4.1: Create a `tui` command in `src/main.rs`.**
    *   This command will launch the TUI interface.
*   **Subtask 4.2: Implement the basic TUI structure in `prompts-tui`.**
    *   Create a simple TUI that lists the available prompts.
*   **Subtask 4.3: Implement prompt selection and display in the TUI.**
*   **Subtask 4.4: Add support for editing and deleting prompts from within the TUI.**

## Phase 3: Desktop Application

### Task 5: Set up the Tauri application

*   **Subtask 5.1: Create the `tauri-app` directory.**
*   **Subtask 5.2: Initialize a new Tauri project.**
*   **Subtask 5.3: Integrate the `prompts-core` library with the Tauri backend.**

### Task 6: Implement the desktop UI

*   **Subtask 6.1: Create a React frontend for the Tauri app.**
*   **Subtask 6.2: Implement features for listing, adding, editing, and deleting prompts in the UI.**

## Phase 4: Documentation and Polish

### Task 7: Improve documentation

*   **Subtask 7.1: Write comprehensive documentation for all CLI commands.**
*   **Subtask 7.2: Create a user guide for the TUI and desktop application.**

### Task 8: Refine error handling

*   **Subtask 8.1: Implement more specific error types using `thiserror`.**
*   **Subtask 8.2: Provide more user-friendly error messages.**
