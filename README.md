# Rust Agentic Skills

**A Constraint-Based AI Coding Environment for Rust.**

This repository implements the "Rust Guild" architecture—a system of specialized AI agents designed to navigate the strict constraints of the Rust compiler. It transforms a standard coding assistant into a disciplined engineering team.

## The Guild Philosophy

Traditional LLMs struggle with Rust because they prioritize **plausibility** over **correctness**. They "hallucinate" lifetimes and unwrap() indiscriminately.

This project solves this by **Steering**:

- **No "One-Size-Fits-All" Agent**: We split intelligence into specialized "Skills".
- **Beast Mode Workflow**: Fetch -> Understand -> Plan -> Implement -> Verify.
- **Compiler-First**: The "Lint Hunter" agent assumes code is broken until `cargo check` passes.

## 📂 Project Structure

```
├── .github/instructions    # The Brains: Skill definitions for the agents
│   ├── rust/               # Rust-specific skills (Core, Lint Hunter, Pest, RON)
│   └── general/            # Universal skills (Debug, Syntax, Security)
├── src/                    # The Code: Where the agents work
├── Gemini.md               # The Architect: Master System Prompt
└── task.md                 # The Memory: Active session state
```

## 🚀 Getting Started

1.  **Load the System Prompt**: Use `Gemini.md` as your agent's system instruction.
2.  **Initialize State**: The agent will create or read `task.md`.
3.  **Command**: Ask for a feature.
    - _User_: "Create a parser for this log file."
    - _Agent_: (Routing...) "Activating Pest Specialist."

## 📚 Documentation

- [AGENTS.md](AGENTS.md): Detailed roster of the available personas.
- [development.md](DEVELOPMENT.md): How to add new skills to the Guild.
- [CONTRIBUTING.md](CONTRIBUTING.md): Guidelines for contributing.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
