#!/bin/bash
# Demo script to verify Gemini interaction with the extension

echo "Setting up Gemini Extension..."

# Try running gemini linkage.
# This assumes 'gemini-cli' is installed (which we do in Dockerfile).
if command -v gemini &> /dev/null; then
    gemini extensions link . || echo "Warning: 'gemini extensions link' failed. (Command exists but might not support extensions)"
elif command -v gemini-cli &> /dev/null; then
    # Try gemini-cli alias if gemini command isn't found directly
    echo "Running with gemini-cli..."
    gemini-cli extensions link . || echo "Note: 'gemini-cli' (PyPI) might not support 'extensions link'. Skipping link step."
else
    echo "Error: gemini command not found."
fi

echo ""
echo "Starting MCP Server (simulating Gemini connection)..."
echo "-----------------------------------------------------"
echo "You can now paste JSON-RPC messages."
echo "-----------------------------------------------------"

./run_rust_guardian.sh
