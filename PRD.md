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

## Technical Architecture

The CLI will be built in Rust. Key libraries will include:
- **Clap**: For command-line argument parsing.
- **Directories**: For finding the appropriate user-specific storage location on different operating systems.
- **Sha2**: for hashing prompt content to create a unique ID.
- **Fuzzy-matcher**: For implementing fuzzy search.
- **Serde**: For serializing and deserializing prompt data.

Prompts will be stored as individual JSON files in a dedicated directory. The filename for each prompt will be the SHA256 hash of its content.

## Feature Specification

- **Storage:**
    - **Default Location:** The tool will use a default directory (e.g., `~/.config/prompts-cli/prompts`).
    - **Custom Location:** A `--config` global flag will allow users to specify an alternative storage directory.
    - **Content-Addressable:** Prompts are stored in files named by the SHA256 hash of their content.
- **Commands:**
    - `add [PROMPT_TEXT]`: Adds a new prompt. If `PROMPT_TEXT` is not provided, it reads from stdin.
    - `list`: Lists all stored prompts, showing a snippet and their hash.
    - `show [FUZZY_QUERY]`: Searches for a prompt. If multiple are found, returns a JSON list. If one is found, it's displayed. If `FUZZY_QUERY` is not provided, it reads from stdin.
    - `edit [FUZZY_QUERY]`: Same search mechanism as `show`. If a single prompt is identified, it opens it for editing (e.g., in the user's `$EDITOR`).
    - `delete [FUZZY_QUERY]`: Same search mechanism as `show`. Deletes the identified prompt after confirmation.
    - `generate [FUZZY_QUERY]`: Same search mechanism as `show`. Generates text based on the identified prompt.
- **Fuzzy Search:**
    - Implemented for `show`, `edit`, `delete`, and `generate`.
    - Matches against the text of the prompts.
    - If multiple matches are found, outputs a JSON array of objects, where each object contains the prompt text and its hash.
- **Prompt Metadata:**
    - The `Prompt` struct will contain the prompt text, tags, and categories. The hash is used as the external identifier.

## Out of Scope for v2

- GUI or TUI interfaces.
- Syncing prompts across devices.
- Advanced versioning of prompts.
