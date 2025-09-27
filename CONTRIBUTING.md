# Contributing

This is a personal learning project, but if you're interested in tinkering with it, here's what you'll need:

## Development Setup

### Prerequisites

- `cargo install cargo-tarpaulin` for coverage reports
- `cargo install tokei` for code statistics

### Commands

- `cargo tarpaulin --out Html` to generate coverage report
- `tokei` to show code statistics

## Code Style

- Run `cargo fmt` before committing as usual
- Ensure `cargo clippy` passes (I have pedantic checks enabled)
