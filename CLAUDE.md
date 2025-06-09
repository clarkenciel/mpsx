# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Codebase Overview

This is a Rust workspace containing reimplementations of common POSIX utilities for educational purposes.

## Architecture

- **Workspace Structure**: Uses Cargo workspace with shared configuration in root `Cargo.toml`
- **Target**: Educational reimplementations of POSIX utilities
- **Edition**: Rust 2024 with optimized release profile (LTO, single codegen unit, panic=abort)

## Development Guidance

### Dependencies Management
- **Always use `cargo add`** to add dependencies, not manual Cargo.toml editing
- For CLI testing, add these dev dependencies: `assert_cmd`, `predicates`, `assert_fs`
- For CLI argument parsing, use `clap` with `derive` feature
- Workspace uses **Rust 2024 edition**

### Integration Testing Best Practices
Based on [Rust CLI Book testing guidance](https://rust-cli.github.io/book/tutorial/testing.html):

**Research Phase:**
- **Always consult the man page** (`man <utility>`) before writing integration tests
- Study the original utility's behavior, options, output format, and edge cases
- Use man page examples to understand expected behavior patterns

**Testing Tools:**
- `assert_cmd::Command` - Run CLI binaries in tests
- `assert_fs::TempDir` - Create temporary test files
- `predicates` - Write flexible test assertions

**Testing Approach:**
- Test **observable user behaviors**, not implementation details
- Focus on **different behavior types** rather than exhaustive edge cases
- Test both **success and failure scenarios**
- Verify compatibility with original utilities by comparing outputs

**Test Coverage Areas:**
- File handling (existing files, non-existent files, empty files)
- Argument processing (short flags, long flags, combinations)
- Error scenarios (file not found, invalid arguments)
- stdin input handling
- Multiple file processing
- Unicode/UTF-8 handling
- Edge cases (whitespace, special characters)

### Project Structure
- **Monorepo workspace** for POSIX utility reimplementations
- Each utility gets its own binary crate: `cargo new <name> --bin`
- Projects are **educational reimplementations** focusing on behavioral compatibility
- Include comprehensive README files for workspace and individual crates
- Use workspace-inherited package metadata in crate Cargo.toml files

### What Agents MUST NOT Do
- **NEVER write implementation code** in `main.rs`, `lib.rs`, or any source files - this project is for human learning
- **DO NOT modify existing implementation code** - only create project structure, tests, and documentation
- **NEVER add implementation logic** to any `.rs` files in `src/` directories
- **DO NOT write function bodies** beyond minimal stubs required for compilation
- The human must write all actual utility logic themselves for educational purposes

## Common Commands

### Building
```bash
cargo build                    # Build all workspace members
cargo build -p mwc            # Build specific project
cargo build --release         # Optimized release build
```

### Testing
```bash
cargo test                     # Run all tests
cargo test -p <crate-name>     # Run tests for specific project
cargo test --test integration_tests  # Run integration tests only
cargo test test_name          # Run specific test
```

### Running
```bash
cargo run -p mwc -- [args]    # Run with arguments
cargo run -p mwc -- --help    # Show help
```

### Development
```bash
cargo check                   # Quick syntax check
cargo clippy                  # Linting
cargo fmt                     # Format code
```

## Project-Specific Notes

### mwc (Word Count Tool)
- CLI interface using `clap` with derive features
- Comprehensive integration test suite covering all standard `wc` functionality
- Tests use regex patterns to validate output format consistency