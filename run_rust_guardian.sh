#!/bin/bash
# Wrapper script to ensure correct CWD and environment for rust-guardian

# Get the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

# Set the working directory to the project root (where the script is)
cd "$SCRIPT_DIR" || exit 1

# Check if the release binary exists
BINARY="./target/release/rust-agentic-skills"

# Ensure cargo is in PATH if not already
if ! command -v cargo &> /dev/null; then
    if [ -f "$HOME/.cargo/bin/cargo" ]; then
        export PATH="$HOME/.cargo/bin:$PATH"
    fi
fi

if [ ! -f "$BINARY" ]; then
    # Fallback to cargo run if binary doesn't exist, but warn about it
    # We write to stderr to avoid polluting stdout (JSON-RPC)
    echo "Warning: Release binary not found at $BINARY. Building..." >&2
    
    if ! command -v cargo &> /dev/null; then
        echo "Error: 'cargo' not found in PATH. Cannot build binary." >&2
        exit 1
    fi

    if ! cargo build --release --bin rust-agentic-skills --quiet >&2; then
        echo "Error: Failed to build rust-agentic-skills binary." >&2
        exit 1
    fi
fi

# Exec the binary, passing all arguments
# We use exec to replace the shell process with the binary
exec "$BINARY" "$@"
