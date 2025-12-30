# Contributing to Rust Agentic Skills

We welcome contributions to the Rust Guild! Whether you're adding a new Skill, fixing a prompt, or improving the orchestrator, here is how to work with us.

## The Prime Directive

**Do not break the Beast Mode loop.**
Any change to the system prompt or skills must preserve the `Fetch -> Plan -> Execute -> Verify` workflow.

## How to Contribute

1.  **Fork & Clone**: Standard GitHub flow.
2.  **Create a Branch**: `feature/new-skill-bevy` or `fix/lint-hunter-E0502`.
3.  **Test Your Skill**:
    - If adding a new skill, create a test scenario (like `src/bin/verify_borrow.rs`) that triggers it.
    - Verify that the Router correctly selects your new skill.
4.  **Submit a PR**:
    - Describe the specific constraint or capability you are adding.
    - Example: "Added `Actix Specialist` to handle async actor lifetimes."

## Style Guide for Instructions

- **Imperative Mood**: Tell the agent what to _do_, not what it _could_ do.
- **XML Tags**: Use `<role_definition>`, `<constraints>`, etc., to structure the prompt.
- **Examples**: Always provide specific code examples of "Bad" vs "Good".
