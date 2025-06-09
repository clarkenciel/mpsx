# MPSX

A Rust monorepo workspace containing various command-line utilities and tools.

These are reimplementations of common POSIX utilities for
educational purposes.

## About

This workspace uses Cargo workspaces to manage multiple related projects in a single repository. Each project is contained in its own directory and can be built, tested, and published independently while sharing common dependencies and configuration.

## Workspace Structure

- `mwc/` - A behavioral reimplementation of the `wc` (word count) utility

## Building

To build all workspace members:

```bash
cargo build
```

To build a specific project:

```bash
cargo build -p mwc
```

## Testing

To run tests for all workspace members:

```bash
cargo test
```

To run tests for a specific project:

```bash
cargo test -p mwc
```

## Running

To run a specific binary:

```bash
cargo run -p mwc -- [arguments]
```

## Development

This workspace uses Rust 2024 edition and follows standard Rust conventions. Each project should include its own README with specific usage instructions.

## License

MIT OR Apache-2.0