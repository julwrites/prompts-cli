# Product Requirements Document (PRD): Prompts CLI v2

This document outlines the requirements for the next version of the Prompts CLI, a tool for managing prompts for large language models (LLMs).

## Executive Summary

The Prompts CLI v2 enhances the developer experience by introducing a more streamlined and powerful workflow for prompt management. Key improvements include automatic storage location management, content-addressable storage using hashes for de-duplication, fuzzy search capabilities, and a more flexible interactive command structure.

This document also outlines the requirements for building a terminal user interface (TUI) and a desktop application for managing prompts for large language models (LLMs) using the Tauri framework.

## Product Overview

### Problem Statement

Developers need a frictionless way to manage their LLM prompts. The previous version of the CLI required manual file management (`--file` parameter), which was cumbersome. Naming prompts was an extra step, and finding them relied on exact matches. This version aims to remove these barriers.

For users who are less comfortable with command-line interfaces, or for those who prefer a more feature-rich graphical environment, a dedicated desktop application is the ideal solution for prompt management.

While a CLI is powerful, some users prefer a more visual and interactive way to browse, search, and manage their prompts. A TUI can provide a more intuitive user experience for these users, while still remaining within the terminal environment.

### Solution Approach

The CLI will now manage its own storage, using a default location within the user's config or data directory. Prompts will be identified by a hash of their content, eliminating the need for manual naming and enabling automatic de-duplication. Commands will support both one-shot and interactive modes, and a fuzzy finder will help users quickly locate prompts.

The Prompts Tauri application will be a desktop application built using Tauri, with a web-based frontend (likely using a framework like React or Vue) and a Rust backend. It will be part of the `prompts-cli` repository, but will be built and distributed as a separate application.

The Prompts TUI will be built in Rust using the Ratatui library. It will be an integrated part of the `prompts-cli` binary, launched with a specific command or flag (e.g., `prompts-cli tui`).

### Target Audience

- Developers who utilize LLMs for software development, content generation, or automation.
- Users who are comfortable working in a command-line environment.
- Anyone who needs to manage a large collection of prompts efficiently.
- Users who prefer a graphical user interface (GUI) for their applications.
- Developers and content creators who want a dedicated and powerful tool for managing their prompts.
- Users who want a consistent prompt management experience across different operating systems (Windows, macOS, and Linux).
- Developers and power users who spend a significant amount of time in the terminal.
- Users who prefer a more visual and interactive experience than a traditional CLI.
- Those who want to manage their prompts without switching to a full graphical desktop application.

## Product Goals and Success Metrics

| Goal | Success Metric | Target |
|---|---|---|
| **Frictionless Prompt Management** | Time to add and retrieve a prompt is significantly reduced. | < 5 seconds per operation |
| **Intuitive User Experience** | High user satisfaction and adoption rates. | >95% positive feedback from users |
| **Robust and Scalable Storage**| The system handles thousands of prompts efficiently without performance degradation. | CRUD operations remain under 100ms with 10,000+ prompts |
| **Cross-platform Compatibility** | Builds and runs seamlessly on Linux, macOS, and Windows. | 100% supported platforms |
| User-friendly prompt management   | High user satisfaction and positive reviews  | High ratings in app stores/feedback channels |
| Native desktop experience         | Seamless integration with the host OS        | Use of native notifications, menus, etc. |
| Cross-platform compatibility      | Builds and runs on Linux/macOS/Windows       | 100% supported platforms       |
| Intuitive prompt management       | User satisfaction and ease of use            | High ratings from user feedback |
| Responsive and performant UI      | Smooth and lag-free user experience          | < 100ms response to user input |
| Seamless integration with CLI     | Easy to launch and exit the TUI from the CLI | Clear and simple commands      |

## User Stories

| Story ID | User Story | Acceptance Criteria | Priority |
|---|---|---|---|
| **US-001** | As a developer, I want the CLI to automatically manage where my prompts are stored so I don't have to think about it. | The CLI uses a default, user-specific directory (e.g., `~/.config/prompts-cli`). The user can override this with a `--config` flag. | P0 |
| **US-002** | As a developer, I want to add prompts without having to name them, and the tool should handle duplicates. | A prompt's content is hashed to create a unique ID. Adding an existing prompt is a no-op. | P0 |
| **US-003** | As a developer, I want to quickly find a prompt even if I only remember parts of it. | Commands that need to identify a prompt use a fuzzy search on the prompt text. | P0 |
| **US-004** | As a developer, I want to either provide a prompt directly in a command or have the CLI ask me for it. | Commands like `add`, `show`, `edit`, `delete` support both a one-shot mode (prompt in args) and an interactive mode (reads from stdin). | P1 |
| **US-005** | As a developer, when my fuzzy search returns multiple results, I want the CLI to show me the options so I can choose the correct one. | The CLI returns a structured JSON list of matching prompts, including their text and hash, for the user to make a specific choice. | P1 |
| US-006 | As a terminal user, I want a TUI to browse and manage prompts interactively. | Responsive Ratatui interface with keyboard navigation. | P1 |
| US-007 | As a desktop app user, I want a user-friendly UI for managing prompts with drag & drop and rich controls. | Tauri app with native integrations and a smoother UX. | P1 |
| **US-008** | As a developer, I want to use variables in my prompts so I can easily reuse them for different contexts. | The `generate` command supports a `--variable "key=value"` syntax. The prompt text can contain `{{key}}` placeholders. | P1 |
| **US-009** | As a developer, I want to organize my prompts with tags so I can easily find all prompts related to a specific topic. | The `add` and `edit` commands support a `--tag` flag. The `list` command can filter by tags. | P1 |
| US-010 | As a TUI user, I want to see a list of my prompts with their titles and tags. | A scrollable list of prompts is displayed on launch. | P1 |
| US-011 | As a TUI user, I want to be able to select a prompt and view its full content. | A dedicated view shows the full text of the selected prompt. | P1 |
| US-012 | As a TUI user, I want to be able to edit a prompt's content and metadata directly within the interface. | An editing mode allows for in-place modification of prompts. | P1 |
| US-013 | As a desktop user, I want to be able to use my mouse to navigate the application and interact with my prompts. | The application is fully navigable with a mouse. | P1 |
| US-014 | As a desktop user, I want to receive native notifications for certain events (e.g., when a prompt is successfully saved). | The application uses the OS's native notification system. | P1 |
| US-015 | As a desktop user, I want to be able to customize the application's appearance (e.g., with a light or dark theme). | The application provides theme customization options. | P2 |
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

The TUI will be built in Rust using the Ratatui and Crossterm libraries. It will be a module within the `prompts-cli` crate and will share the same core logic for prompt management.

The desktop application will be built using the Tauri framework. The frontend will be a single-page application (SPA) built with a modern web framework like React or Vue. The backend will be written in Rust and will be responsible for all the core prompt management logic.

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
- **Interactive Prompt List**: A scrollable and filterable list of all prompts.
- **Prompt Content View**: A detailed view of the selected prompt's content and metadata.
- **In-TUI Editing**: The ability to edit prompts directly within the TUI.
- **Keyboard-driven Navigation**: Intuitive keybindings for all actions.
- **Help and Documentation**: A help screen that explains the keybindings and features.
- **Graphical Prompt Management**: All the core CRUD, search, and filtering features will be available through a user-friendly GUI.
- **Rich Text Editing**: A "what you see is what you get" (WYSIWYG) editor or a code editor with syntax highlighting for editing prompts.
- **Drag and Drop**: The ability to drag and drop prompts to reorder them or organize them into folders.
- **Native OS Integration**: Use of native menus, notifications, and other OS-specific features.
- **Customization**: Options to customize the application's appearance and behavior.

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

- GUI or TUI interfaces.
- Syncing prompts across devices.
- Advanced versioning of prompts.
- Automatic syncing of prompts between devices.