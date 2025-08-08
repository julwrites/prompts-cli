# Agent Instructions for `prompts-cli`

This document provides guidance for LLM agents and developers working on the `prompts-cli` repository.

## Project Overview

This repository contains the core logic and the command-line interface (CLI) for the Prompts project. It is a Rust-based tool built with a library-first approach. The core logic is encapsulated in a library crate, which is then used by the CLI binary crate in this repository, as well as by the TUI, Neovim, and Tauri frontends in other repositories.

## Core Tenets

- **Rust for Performance and Safety:** The core logic and frontend are written in Rust to ensure high performance, memory safety, and cross-platform compatibility.
- **API Stability is Paramount:** The public API of the library crate is a critical dependency for all other frontends. Any changes to this API must be made with extreme care and be accompanied by corresponding updates in the other repositories.
- **Test-Driven Development (TDD):** All new features must be accompanied by a comprehensive suite of tests. Development follows a strict TDD workflow where every new feature begins with a failing test.
- **User-Centric Design:** The CLI is designed with the user in mind, providing clear and concise output, helpful error messages, and comprehensive documentation.
- **Cross-Platform Compatibility:** The application must build, run, and pass all tests on Linux, macOS, and Windows.

## Development Workflow

The development process follows the "Red-Green-Refactor" cycle of TDD:

1.  **Understand the Core Library:** Before making any changes, familiarize yourself with the core library crate and its public API.
2.  **Write a Failing Test:** For any new feature or bug fix, write a test that clearly defines the desired behavior and fails because the feature is not yet implemented.
3.  **Implement the Feature:** Write the minimum amount of code required to make the test pass.
4.  **Refactor:** With the safety of a passing test suite, refactor the code to improve its design, readability, and performance.
5.  **Ensure All Tests Pass:** Run all tests to ensure that your changes have not introduced any regressions.
6.  **Update Dependent Repositories:** If you have made any changes to the public API of the library crate, you must update the other repositories to reflect these changes.

## Key Technologies

- **Rust:** The primary programming language.
- **Clap:** For parsing command-line arguments.
- **Serde:** For serializing and deserializing data.
- **anyhow & thiserror:** For ergonomic and structured error handling.

## Testing Strategy

- **Unit Tests:** The core library crate must have extensive unit tests for every function. This includes testing the storage logic, templating engine, and fuzzy search algorithm in isolation.
- **Integration Tests:** A dedicated test suite tests the CLI binary itself. This involves running the compiled binary with various commands and arguments and asserting on the output and exit codes. The `assert_cmd` and `predicates` crates are used for this.
- **Cross-Platform Testing:** The CI/CD pipeline runs all tests on Linux, macOS, and Windows to ensure cross-platform compatibility.
- **Storage Layer Mocking:** To test application logic without interacting with the file system, the storage layer is mocked using the `mockall` crate.

## Key Commands

- **Run unit tests:** `cargo test --lib`
- **Run integration tests:** `cargo test --test '*' -- --nocapture`
- **Run all tests:** `cargo test --all-targets`
- **Build the project:** `cargo build`