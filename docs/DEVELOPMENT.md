# Development Guide: Extending the Guild

This guide explains how to add new capabilities to the Rust Agentic system.

## Anatomy of a Skill

A Skill is a Markdown file in `.github/instructions/` that follows this schema:

1.  **Role Definition**: Who is this agent? (e.g., "You are the SQL Specialist").
2.  **Triggers**: When should the Router call this agent?
3.  **Philosophy/Constraints**: Hard rules (e.g., "Always use prepared statements").
4.  **Patterns**: Concrete code snippets.

## Adding a New Skill

**Scenario**: You want to add support for `Axum` web server development.

1.  **Create the File**: `.github/instructions/rust/axum.instructions.md`.
2.  **Define the Role**:
    ```markdown
    <role_definition>
    You are the Axum Specialist. You build async web handlers.
    </role_definition>
    ```
3.  **Update the Router**:
    - Edit `.github/instructions/rust/multi-agent/index.instructions.md`.
    - Add: `Keywords: "http", "api", "rest", "axum" -> Route: Axum Specialist`.
4.  **Verify**:
    - Start a session.
    - Ask: "Create a Hello World API."
    - Verify the agent outputs: `> ROUTING: Axum Specialist`.

## Debugging Prompts

If the agent is ignoring your instructions:

- **Too much noise?** Remove "fluff" text. Keep it telegraphic.
- **Conflicting constraints?** Check if `Rust Core` is overriding your specific rule. Specific (Skill) should always override General (Core).
