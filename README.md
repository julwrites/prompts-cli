# Prompts CLI

A command-line tool for creating, storing, managing, and finding prompts for large language models (LLMs).

## Overview

The Prompts CLI is designed to provide a frictionless, powerful, and intuitive workflow for developers to manage their LLM prompts. It offers a robust set of features to streamline the process of adding, retrieving, and using prompts.

The project is built in Rust with a "library-first" approach, where the core logic is encapsulated in a library crate and the command-line interface is a separate binary crate. This modular design facilitates testing and allows the core logic to be reused by other frontends.

## Key Features

- **Automatic Storage Management**: The CLI manages its own storage, using a default location within the user's config or data directory.
- **Content-Addressable Storage**: Prompts are identified by a hash of their content, eliminating the need for manual naming and enabling automatic de-duplication.
- **Fuzzy Search**: Quickly find any prompt using fuzzy search, even if you only remember parts of it.
- **Tagging and Categorization**: Organize your prompts with tags for easy filtering and retrieval.
- **Templating System**: Reuse prompts for different contexts with a simple templating system.

## Installation

To install the Prompts CLI, you will need to have Rust and Cargo installed. You can then build the project from the source:

```bash
cargo build --release
```

The executable will be located at `target/release/prompts-cli`.

## Usage

Here are some of the key commands available in the Prompts CLI:

- **Add a new prompt:**
  ```bash
  prompts-cli add "This is a new prompt."
  ```

- **List all prompts:**
  ```bash
  prompts-cli list
  ```

- **Show a prompt using fuzzy search:**
  ```bash
  prompts-cli show "new prompt"
  ```

- **Edit a prompt:**
  ```bash
  prompts-cli edit "new prompt"
  ```

- **Delete a prompt:**
  ```bash
  prompts-cli delete "new prompt"
  ```

- **Generate text from a prompt with variables:**
  ```bash
  prompts-cli generate "prompt with {{variable}}" --variable "variable=value"
  ```

For more detailed information on development and contribution, please see `AGENTS.md`.
