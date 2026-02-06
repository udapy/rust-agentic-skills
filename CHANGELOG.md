# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.2] - 2026-02-06

### Fixed

- **Protocol Violation**: Fixed stdout pollution by redirecting build and application logs to stderr (`bootstrap.sh`, `env_logger`).
- **Initialization Hang**: Fixed JSON-RPC compliance by preventing `null` fields in response objects.
- **Portability**: Updated `gemini-extension.json` to use `${extensionPath}` for global compatibility.

### Added

- **Documentation**: Added architecture diagram and real-world session trace (`examples/gemini_session_trace.md`).

## [1.1.1] - 2026-01-25

### Added

- **Dynamic MCP Server**: Implemented a full Model Context Protocol server in `src/main.rs`.
  - Automatically scans `skills/*/SKILL.md` to register tools.
  - Handles JSON-RPC handshake (`initialize`, `notifications/initialized`).
  - Implemented `tools/list` and `tools/call` parsing.
- **Skill Loader**: New module `src/skills.rs` to deserialize skill metadata and map them to executable scripts.
- **Pollution Protection**: Enforced `Stdio::piped()` for all child process executions to prevent tool output from corrupting the JSON-RPC stream.

### Fixed

- **Gemini Connection Crash**: Resolved `MCP error -32000: Connection closed` by providing a valid server implementation that holds the connection open (Fixes Issue #3).
- **Ambiguous Execution**: Updated `gemini-extension.json` to explicitly use `--bin rust-agentic-skills`, preventing `cargo run` from executing test binaries or scripts accidentally.
- **Dependency Hygiene**: Synced dependencies with upstream requirements (`tokio` full features, `anyhow`) while adding `serde_yaml` for metadata parsing.

### Changed

- Refactored `src/main.rs` from a static placeholder to a dynamic host.
- Updated project architecture to treat the file system (`skills/`) as the source of truth for agent capabilities.
