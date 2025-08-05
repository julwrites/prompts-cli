# Evaluation of the `prompts-cli` Application

This document provides a detailed evaluation of the `prompts-cli` application, comparing its current implementation against the requirements outlined in the Product Requirements Document (`PRD.md`).

## 1. Project Summary

The `prompts-cli` is a command-line tool designed to help developers manage a collection of prompts for Large Language Models (LLMs). The goal is to provide a frictionless, powerful, and intuitive workflow for adding, retrieving, and using prompts. Key features specified in the PRD include automatic storage management in a user-specific directory, content-addressable storage to prevent duplicates, fuzzy search for easy retrieval, tagging and categorization, and a templating system for prompt reuse.

## 2. High-Level Assessment

The project is well-structured and follows modern Rust development practices. The "library-first" approach, with core logic in a separate `prompts-cli` library crate and the command-line interface in a binary crate, is excellent. This modular design facilitates testing and allows the core logic to be reused by other frontends (TUI, Tauri, etc.), as mentioned in `AGENTS.md`.

The use of established libraries like `clap`, `serde`, `tokio`, and `fuzzy-matcher` is appropriate. The storage layer is abstracted behind a `Storage` trait, which is a great architectural choice that allows for multiple backends (`JsonStorage`, `LibSQLStorage`) and makes the system more flexible and testable.

Overall, the codebase is clean, well-organized, and provides a solid foundation. However, there are several gaps where the current implementation does not fully meet the requirements of the PRD.

## 3. Gap Analysis

The following is a detailed breakdown of discrepancies between the PRD and the implementation, organized by user story.

### US-001 & US-017: Automatic Storage and Configuration

- **Requirement**: The CLI should use a default, user-specific directory (e.g., `~/.config/prompts-cli`) for both storage and the `config.toml` file. This location should be overridable.
- **Implementation**:
    - The `storage.rs` module correctly implements a `get_default_storage_dir` function using the `dirs` crate to find the standard config directory. The `JsonStorage` and `LibSQLStorage` structs correctly use this function as a fallback.
    - However, `main.rs` does not use this logic for loading `config.toml`. It only looks for the configuration file in the current working directory, completely ignoring the user's config directory.
- **Gap**: The application fails to load the configuration file from the standard, user-specific location, making the configuration feature unreliable and not compliant with the PRD.

### US-002: Content-Addressable Storage & De-duplication

- **Requirement**: Prompts are identified by a hash of their content. Adding an existing prompt should be a no-op (i.e., the tool should handle duplicates gracefully).
- **Implementation**: The `Prompt::new` function correctly generates a SHA256 hash of the content. The storage backends use this hash for identification.
- **Gap**: The de-duplication logic is incomplete.
    - `JsonStorage::save_prompt` will simply overwrite an existing prompt file if the hash is the same. This is not a "no-op" and can lead to unintended data changes if metadata (like tags) was different.
    - `LibSQLStorage::save_prompt` will likely return a "UNIQUE constraint failed" error if a prompt with the same hash is added, causing the application to crash instead of handling it gracefully.
    - The `add_prompt` function in `Prompts` does not check for existence before saving.

### US-003: Fuzzy Search

- **Requirement**: Commands like `show`, `edit`, `delete`, and `generate` should use fuzzy search to find prompts.
- **Implementation**: The `core::search_prompts` function correctly uses the `fuzzy-matcher` library.
- **Gap**: No gap found. This appears to be implemented correctly.

### US-009: Tagging and Filtering

- **Requirement**: The `add` and `edit` commands should support adding tags (`--tag`). The `list` command should be able to filter prompts by tag.
- **Implementation**:
    - The `add` and `edit` commands in `main.rs` have a `--tag` flag.
    - The `core::search_prompts` function includes logic to filter by tags and categories.
- **Gap**: The feature is incomplete. The `list` command in `main.rs` does not expose any flags for filtering by tag. Furthermore, the `Prompts::show_prompt` API call in `core/mod.rs` does not pass any tags or categories to the underlying `search_prompts` function, meaning that none of the search-based commands (`show`, `edit`, `delete`, `generate`) can actually filter by tag, even though the capability exists in the search function itself.

### Architectural Considerations: Structured Error Handling

- **Requirement**: "When the `--output json` flag is used, all errors should be returned as a structured JSON object with a clear error message and a unique error code."
- **Implementation**: The application currently uses `anyhow::Result` and prints human-readable error messages to `stderr`. There is no mechanism for structured JSON error output.
- **Gap**: The application does not meet the requirement for structured JSON error handling, which is important for scripting and integration with other tools.

## 4. Recommendations

Based on the analysis, the following actions are recommended to bring the application in line with the PRD and improve its quality:

1.  **Fix Configuration Loading**: Modify `main.rs` to use the `get_default_storage_dir` function to locate and load `config.toml` from the user's default configuration directory.
2.  **Implement Proper De-duplication**: In `Prompts::add_prompt`, check if a prompt with the same hash already exists before attempting to save. If it exists, the operation should be a true no-op.
3.  **Complete the Tagging Feature**:
    - Add a `--tag` option to the `list` command in `main.rs` to allow filtering.
    - Update the `Prompts::list_prompts` function to accept optional tag filters and pass them to `search_prompts`.
    - Modify `Prompts::show_prompt` and other search-based commands to accept and use tag/category filters.
4.  **Implement Structured Error Handling**: Introduce a global `--output json` flag. When this flag is present, `main` should catch any `Err` results and print a serialized JSON error object to `stdout` or `stderr` instead of the default `anyhow` report.
5.  **Improve `edit` Command Ergonomics**: The current `edit` command requires the user to re-specify all tags and categories. It should instead fetch the existing prompt and merge the changes, allowing users to add or remove specific tags without re-entering the entire list.
6.  **Enhance Test Coverage**: While `AGENTS.md` mandates TDD, it would be beneficial to add integration tests specifically for the failure cases identified in this evaluation (e.g., adding a duplicate prompt, loading config from the correct path).
