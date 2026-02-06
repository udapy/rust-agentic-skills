#!/bin/bash
# Bootstrap script to build silently and run the MCP server
# Validated for MCP Protocol Compliance (StdOut Cleaning) + Distribution

# 1. Resolve Directory (Handles Symlinks/Relative Paths)
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
cd "$SCRIPT_DIR" || exit 1

# 2. Ensure Cargo is in PATH (Common issue in cron/shell environments)
if ! command -v cargo &> /dev/null; then
    if [ -f "$HOME/.cargo/bin/cargo" ]; then
        export PATH="$HOME/.cargo/bin:$PATH"
    fi
fi

# 3. Build (if needed) - SILENTLY to stderr
# We use >&2 to send any potential stdout from cargo to stderr
# We only build if the binary is missing or if explicitly requested (e.g. dev mode)
BINARY="./target/release/rust-agentic-skills"
if [ ! -f "$BINARY" ]; then
    echo "Building Rust Agentic Skills (First Run)..." >&2
    if ! cargo build --release --quiet >&2; then
        echo "Error: Build failed. Check stderr for details." >&2
        exit 1
    fi
fi

# 4. Exec the binary directly (replacing this shell process)
# This preserves PID and signal handling
exec "$BINARY"
