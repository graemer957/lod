# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`lod` (Laptop or Desktop) is a macOS menu bar utility written in Rust that manages system settings based on whether a laptop is docked or not. It uses AppleScript to change system preferences and can keep the machine awake using `caffeinate`.

**Key characteristics:**
- Personal learning project focused on writing macOS GUI apps in Rust
- Intentionally lightweight - not production-grade robustness
- Uses Objective-C bindings (objc2 crates) to interact with macOS APIs
- Uses a forked version of `system_status_bar_macos` with extended functionality

## Development Commands

### Building and Testing
```bash
# Run all tests
cargo test

# Build release binary
cargo build --release

# Run the app locally (macOS only)
cargo run
```

### Code Quality
```bash
# Format code (required before commits)
cargo fmt

# Run linter (pedantic checks enabled - must pass)
cargo clippy
```

### Coverage and Statistics
```bash
# Generate code coverage report (requires cargo-tarpaulin)
cargo tarpaulin --out Html

# View code statistics (requires tokei)
tokei
```

## Architecture

### Core Components

**AppState** (`src/app_state.rs`)
- Central state manager for the application
- Manages current mode (Laptop/Desktop) via `Mode` enum
- Handles caffeination process lifecycle via `WaitingChild`
- Configures menu items dynamically based on state
- Sends `StateChangeMessage` events via mpsc channel

**Application** (`src/application.rs`)
- Custom event loop implementation using macOS NSApplication
- Based on `system_status_bar_macos` but modified to avoid CPU waste
- Polls for macOS events and processes state change messages
- Lives on the main thread (required by macOS APIs)

**Config** (`src/config.rs`)
- Loads configuration from `~/.config/lod/config.toml`
- Creates default config if missing
- Manages temporary AppleScript files using tempfile crate
- AppleScripts are written to temp files at runtime and cleaned up on drop

**Program** (`src/program.rs`)
- Abstraction over `Command` execution with testability via mockall
- Validates exit codes against expected values
- Used for running `osascript`, `defaults`, and `caffeinate`

**WaitingChild** (`src/waiting_child.rs`)
- Spawns background thread to monitor child processes
- Sends `ClearCaffeination` message when caffeinate process exits
- Enables async cleanup without blocking the event loop

### Event Flow

1. Main thread creates mpsc channel and initializes `AppState`
2. `Application::run` starts event loop, listening for both macOS events and state messages
3. User clicks menu items → sends `StateChangeMessage` via channel
4. Main loop receives message → calls callback → mutates `AppState`
5. `AppState` updates menu items and runs AppleScripts as needed

### macOS Integration

**Status Bar**
- Uses `system_status_bar_macos` crate (forked version with extensions)
- SF Symbols for icons: "laptopcomputer" and "desktopcomputer"
- Menu items created dynamically based on current state

**AppleScript Execution**
- AppleScripts from config are written to temp files (needed by `osascript`)
- Executed via `osascript <temp-file-path>`
- Run in background thread to avoid blocking UI

**Dock Detection**
- Determines initial mode by reading `com.apple.dock autohide` preference
- Uses `defaults read` command via the `Program` abstraction

## Configuration

The app expects `~/.config/lod/config.toml` with:
```toml
desktop_applescript = "<AppleScript to run when switching to Desktop>"
laptop_applescript = "<AppleScript to run when switching to Laptop>"

# Optional
caffeinate_app = "<Path to custom binary for keeping machine awake>"
caffeinate_options = "<Options to pass to caffeinate binary>"
```

Default config is automatically created from `example-config.toml` if missing.

## Testing Considerations

- Tests use `mockall` for mocking `Command` trait
- Most macOS-specific code is behind `#[cfg(target_os = "macos")]`
- Linux build has empty main function to allow CI/CD on non-macOS platforms
- Integration tests in `tests/program.rs`

## Clippy Configuration

This project uses **pedantic** and **nursery** clippy lints. Code must pass:
```bash
cargo clippy
```

Some specific allows in the codebase:
- `#[allow(clippy::nonminimal_bool)]` in caffeination toggle (negation improves clarity)
- `#[allow(clippy::module_name_repetitions)]` for `ProgramImpl`
