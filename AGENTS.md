# Agent Instructions for `prompts-cli`

This document provides guidance for LLM agents working on the `prompts-cli` repository.

## Project Overview

This repository contains the core logic and the command-line interface (CLI) for the Prompts project. It is a Rust-based tool built with a library-first approach. The core logic is encapsulated in a library crate, which is then used by the CLI binary crate in this repository, as well as by the TUI, Neovim, and Tauri frontends in other repositories.

## Core Tenets

- **API Stability is Paramount:** The public API of the library crate is a critical dependency for all other frontends. Any changes to this API must be made with extreme care and be accompanied by corresponding updates in the other repositories.
- **Test-Driven Development (TDD):** All new features must be accompanied by a comprehensive suite of tests. This includes unit tests for the library crate and integration tests for the CLI binary.
- **Cross-Platform Compatibility:** The application must build and run on Linux, macOS, and Windows. All tests must pass on all three platforms.

## Development Workflow

1.  **Understand the Core Library:** Before making any changes, familiarize yourself with the core library crate and its public API.
2.  **Write Tests First:** For any new feature, write a failing test that clearly defines the desired behavior.
3.  **Implement the Feature:** Write the minimum amount of code required to make the test pass.
4.  **Run All Tests:** Ensure that all tests, including unit and integration tests, pass on all supported platforms.
5.  **Update Dependent Repositories:** If you have made any changes to the public API of the library crate, you must update the other repositories to reflect these changes.

## Key Commands

- **Run unit tests:** `cargo test --lib`
- **Run integration tests:** `cargo test --test '*' -- --nocapture`
- **Run all tests:** `cargo test --all-targets`
- **Build the project:** `cargo build`