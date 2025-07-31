# Gemini Development Guide: Prompts CLI

This document guides the development of the Prompts CLI tool, a multi-frontend Rust application for prompt management.

## Project Overview

The project is a Rust-based CLI tool with multiple frontends (CLI, TUI, Tauri, Neovim plugin). The core logic is in the `prompts-cli` crate, and each frontend is a separate crate that uses the core crate. This allows for a modular and maintainable architecture.

## Core Tenets

- **Rust for Performance:** The core logic and all frontends will be written in Rust to ensure high performance and cross-platform compatibility.
- **TDD Workflow:** Development will follow a strict Test-Driven Development (TDD) approach. Every new feature will start with a failing test.
- **Multi-Frontend Architecture:** The application will have a core library crate and multiple binary crates for the different frontends. This will keep the concerns separate and the codebase clean.
- **User-Centric Design:** The CLI will be designed with the user in mind, providing clear and concise output, helpful error messages, and comprehensive documentation.
- **Cross-Platform Compatibility:** The application will be tested on Linux, macOS, and Windows to ensure it works seamlessly across all major platforms.

## Development Workflow

1.  **Test First:** For any new feature, write a failing test that clearly defines the desired behavior.
2.  **Implement:** Write the minimum amount of code required to make the test pass.
3.  **Refactor:** Refactor the code to improve its design, readability, and performance, ensuring all tests still pass.
4.  **Repeat:** Repeat the cycle for the next feature.

## Key Technologies

- **Rust:** The primary programming language.
- **Clap:** For parsing command-line arguments.
- **anyhow & thiserror:** For error handling.
- **Serde:** For serializing and deserializing data.

## Initial Setup

1.  **Initialize Cargo Project:** Set up a new Cargo project with a library crate for the core logic and a binary crate for the CLI frontend.
2.  **Add Dependencies:** Add `clap` for argument parsing and `anyhow` and `thiserror` for error handling.
3.  **Write First Test:** Write a simple test for the CLI to ensure that the basic command-line parsing is working correctly.