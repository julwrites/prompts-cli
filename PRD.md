# Product Requirements Document (PRD): Prompts CLI v2

This document outlines the requirements for the next version of the Prompts CLI, a tool for managing prompts for large language models (LLMs).

## Executive Summary

The Prompts CLI v2 enhances the developer experience by introducing a more streamlined and powerful workflow for prompt management. Key improvements include automatic storage location management, content-addressable storage using hashes for de-duplication, fuzzy search capabilities, and a more flexible interactive command structure.

## Product Overview

### Problem Statement

Developers need a frictionless way to manage their LLM prompts. The previous version of the CLI required manual file management (`--file` parameter), which was cumbersome. Naming prompts was an extra step, and finding them relied on exact matches. This version aims to remove these barriers.

### Solution Approach

The CLI will now manage its own storage, using a default location within the user's config or data directory. Prompts will be identified by a hash of their content, eliminating the need for manual naming and enabling automatic de-duplication. Commands will support both one-shot and interactive modes, and a fuzzy finder will help users quickly locate prompts.

### Target Audience

- Developers who utilize LLMs for software development, content generation, or automation.
- Users who are comfortable working in a command-line environment.
- Anyone who needs to manage a large collection of prompts efficiently.

## Product Goals and Success Metrics

| Goal | Success Metric | Target |
|---|---|---|
| **Frictionless Prompt Management** | Time to add and retrieve a prompt is significantly reduced. | < 5 seconds per operation |
| **Intuitive User Experience** | High user satisfaction and adoption rates. | >95% positive feedback from users |
| **Robust and Scalable Storage**| The system handles thousands of prompts efficiently without performance degradation. | CRUD operations remain under 100ms with 10,000+ prompts |
| **Cross-platform Compatibility** | Builds and runs seamlessly on Linux, macOS, and Windows. | 100% supported platforms |

## User Stories

| Story ID | User Story | Acceptance Criteria | Priority |
|---|---|---|---|
| **US-001** | As a developer, I want the CLI to automatically manage where my prompts are stored so I don't have to think about it. | The CLI uses a default, user-specific directory (e.g., `~/.config/prompts-cli`). The user can override this with a `--config` flag. | P0 |
| **US-002** | As a developer, I want to add prompts without having to name them, and the tool should handle duplicates. | A prompt's content is hashed to create a unique ID. Adding an existing prompt is a no-op. | P0 |
| **US-003** | As a developer, I want to quickly find a prompt even if I only remember parts of it. | Commands that need to identify a prompt use a fuzzy search on the prompt text. | P0 |
| **US-004** | As a developer, I want to either provide a prompt directly in a command or have the CLI ask me for it. | Commands like `add`, `show`, `edit`, `delete` support both a one-shot mode (prompt in args) and an interactive mode (reads from stdin). | P1 |
| **US-005** | As a developer, when my fuzzy search returns multiple results, I want the CLI to show me the options so I can choose the correct one. | The CLI returns a structured JSON list of matching prompts, including their text and hash, for the user to make a specific choice. | P1 |
| **US-008** | As a developer, I want to use variables in my prompts so I can easily reuse them for different contexts. | The `generate` command supports a `--variable "key=value"` syntax. The prompt text can contain `{{key}}` placeholders. | P1 |
| **US-009** | As a developer, I want to organize my prompts with tags so I can easily find all prompts related to a specific topic. | The `add` and `edit` commands support a `--tag` flag. The `list` command can filter by tags. | P1 |
| **US-016** | As a developer, I want to be able to import and export my prompts so I can share them with others or back them up. | The CLI has `import` and `export` commands that can handle a directory of prompt files. | P2 |
| **US-017** | As a developer, I want to configure the CLI using a configuration file so I don't have to pass the same flags every time. | The CLI reads `~/.config/prompts-cli/config.toml` for default settings. | P2 |

## Technical Architecture

The CLI will be built in Rust. Key libraries will include:
- **Clap**: For command-line argument parsing.
- **Directories**: For finding the appropriate user-specific storage location on different operating systems.
- **Sha2**: for hashing prompt content to create a unique ID.
- **Fuzzy-matcher**: For implementing fuzzy search.
- **Serde**: For serializing and deserializing prompt data.
- **Tera**: For prompt templating.

Prompts will be stored as individual JSON files in a dedicated directory. The filename for each prompt will be the SHA256 hash of its content.

## Feature Specification

- **Storage:**
    - **Default Location:** The tool will use a default directory (e.g., `~/.config/prompts-cli/prompts`).
    - **Custom Location:** A `--config` global flag will allow users to specify an alternative storage directory.
    - **Content-Addressable:** Prompts are stored in files named by the SHA256 hash of their content.
- **Configuration:**
    - A configuration file at `~/.config/prompts-cli/config.toml` can be used to set default values for the storage path, editor, and other settings.
- **Commands:**
    - `add [PROMPT_TEXT]`: Adds a new prompt. If `PROMPT_TEXT` is not provided, it reads from stdin. Can add tags with `--tag`.
    - `list [--tag TAG]`: Lists all stored prompts, showing a snippet and their hash. Can be filtered by tag.
    - `show [FUZZY_QUERY]`: Searches for a prompt. If multiple are found, returns a JSON list. If one is found, it's displayed. If `FUZZY_QUERY` is not provided, it reads from stdin.
    - `edit [FUZZY_QUERY]`: Same search mechanism as `show`. If a single prompt is identified, it opens it for editing (e.g., in the user's `$EDITOR`).
    - `delete [FUZZY_QUERY]`: Same search mechanism as `show`. Deletes the identified prompt after confirmation.
    - `generate [FUZZY_QUERY] [--variable "key=value"]`: Same search mechanism as `show`. Generates text based on the identified prompt, filling in any variables.
    - `import [DIRECTORY]`: Imports all prompts from a directory.
    - `export [DIRECTORY]`: Exports all prompts to a directory.
- **Fuzzy Search:**
    - Implemented for `show`, `edit`, `delete`, and `generate`.
    - Matches against the text of the prompts.
    - If multiple matches are found, outputs a JSON array of objects, where each object contains the prompt text and its hash.
- **Prompt Metadata:**
    - The `Prompt` struct will contain the prompt text, tags, and categories. The hash is used as the external identifier.
- **Templating:**
    - Prompts can contain variables in the format `{{key}}`.
    - The `generate` command can fill these variables using the `--variable` flag.

## Architectural Considerations

- **Storage Layer Abstraction:** The current plan to use individual JSON files is a good starting point, but it may not be performant for searching and filtering as the number of prompts grows. The storage mechanism should be abstracted behind a trait-based interface to allow for future implementations (e.g., SQLite) without requiring changes to the core application logic.
- **Core Library API:** The core logic will be exposed as a Rust library, which is a critical dependency for the CLI, TUI, and Tauri frontends. This API needs to be well-defined, stable, and thoroughly documented. Any breaking changes to this API will require coordinated updates across all frontends.
- **`generate` Command Scope:** The `generate` command's functionality needs to be clarified. If it is intended to interact with external LLMs, this will introduce significant complexity, including network latency, API key management, and asynchronous handling. For v2, it is recommended that `generate` focuses solely on template rendering.
- **Structured Error Handling:** For scripting and integration with other tools, the CLI must provide predictable error handling. When the `--output json` flag is used, all errors should be returned as a structured JSON object with a clear error message and a unique error code.

## Testing Strategy

- **Unit Tests:** The core library crate must have extensive unit tests for every function. This includes testing the storage logic, templating engine, and fuzzy search algorithm in isolation. Test coverage should be maintained at a high level (e.g., > 90%).
- **Integration Tests:** A dedicated test suite will be created to test the CLI binary itself. This will involve running the compiled binary with various commands, arguments, and flags, and then asserting on the output, exit codes, and any changes to the file system. The `assert_cmd` and `predicates` crates are recommended for this purpose.
- **Cross-Platform Testing:** The CI/CD pipeline must be configured to run all tests on Linux, macOS, and Windows to ensure cross-platform compatibility. GitHub Actions is recommended for this purpose.
- **Storage Layer Mocking:** To test the application logic without interacting with the actual file system, the storage layer will be mocked. The `mockall` crate is recommended for this purpose.

## Out of Scope for v2

- Syncing prompts across devices.
- Advanced versioning of prompts.
- Automatic syncing of prompts between devices.