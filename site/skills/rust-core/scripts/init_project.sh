#!/bin/bash
# Initialize a robust Rust project

if [ -f "Cargo.toml" ]; then
    echo "Cargo projects already exists."
else
    cargo init
fi

# Add standard dependencies
cargo add tokio --features full
cargo add anyhow
cargo add thiserror
cargo add serde --features derive
cargo add tracing
cargo add tracing-subscriber

echo "Project initialized with Tokio, Anyhow, Thiserror, Serde, and Tracing."
