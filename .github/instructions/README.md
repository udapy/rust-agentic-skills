# Rustic Prompt Instructions

This directory contains the "Skills" for the Rust Guild agentic system.

## Structure

### `rust/` (Language Specific)

- `rust.instructions.md`: Core idiomatic Rust generation.
- `linthunter.instructions.md`: Compiler error resolution strategies.
- `pest.instructions.md`: PEG parser generation.
- `ron.instructions.md`: Configuration and Serde handling.
- `multi-agent/index.instructions.md`: The Router logic.

### `general/` (Language Agnostic)

- `debug.instructions.md`: Systematic debugging (Wolf Fence, Binary Search).
- `syntaxhunter.instructions.md`: Syntax error classification.
- `security-github.instructions.md`: Safety and vulnerability auditing.

## Usage

These files are intended to be loaded into the Agent's context dynamically based on the active task or routing decision.
