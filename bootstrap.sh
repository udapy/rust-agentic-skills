#!/bin/bash
# Bootstrap script to build silently and run the MCP server

# Get the directory of this script
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# 1. Build silently to stderr (redirect stdout to null/stderr)
# We use >&2 to send any potential stdout from cargo to stderr
(cd "$DIR" && cargo build --release --quiet >&2)

# 2. Exec the binary directly (replacing this shell process)
cd "$DIR"
exec "$DIR/target/release/rust-agentic-skills"
