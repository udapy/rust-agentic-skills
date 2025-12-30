# Rust Agentic Skills

[![Rust Guild](https://img.shields.io/badge/Agent-Rust%20Guild-orange)](https://github.com/rust-lang/rust)
[![RPI Methodology](https://img.shields.io/badge/Methodology-RPI-blue)](AGENTS.md)
[![Claude Ready](https://img.shields.io/badge/Claude-Plugin-purple)](.Claude-plugin/marketplace.json)
[![Gemini Ready](https://img.shields.io/badge/Gemini-Extension-blue)](.Gemini-extension.json)
[![Antigravity Compatible](https://img.shields.io/badge/Antigravity-Engine-green)](AGENTS.md)

**A modular, constraint-based skill set for Autonomous AI coding agents.**

This repository transforms any general-purpose LLM (Claude, Gemini, GPT-4) into a **disciplined Rust engineering team**. It adheres to the **Agent Context Protocol (ACP)** to provide self-describing skills that explicitly define their triggers, capabilities, and execution phases.

---

## 🏗️ The "Triad" Architecture

We do not use monolithic instruction files. Instead, every skill in `skills/` follows the **Brain-Tool-Context** architecture to maximize token efficiency:

1.  **🧠 Driver (`SKILL.md`)**:
    - **The Brain**. Contains minimal YAML metadata (`trigger`, `rpi_phase`) and high-level role benchmarks.
    - _Usage_: The agent reads this first to decide if it is relevant.
2.  **🛠️ Tools (`scripts/`)**:
    - **The Hands**. Executable code (Shell scripts, Rust binaries) for reliable, deterministic task execution.
    - _Usage_: The agent executes these to perform work (e.g., `init_project.sh`, `explain_error.sh`).
3.  **📚 Context (`references/`)**:
    - **The Knowledge**. Deep-dive documentation and "Dictionaries of Pain".
    - _Usage_: "Lazy loaded" by the agent only when specifically needed to solve a complex problem.

---

## 🤖 Integration Guide

### 1. Claude Code (Plugin)

This repository is a compliant Claude Plugin.

1.  Clone this repository locally.
2.  Allow Claude to discover the `.Claude-plugin/marketplace.json` manifest.
3.  **Result**: Claude will automatically see "Rust Kernel" and "Lint Hunter" as available tools.

### 2. Gemini CLI (Extension)

Compatible with the Gemini CLI tool integration.

1.  Register the extension using `.Gemini-extension.json`.
2.  **Result**: The router skill becomes the entry point for "Rust Guild" commands.

### 3. Manual / Custom Agents

For generic agents (ChatGPT, heavily customized setups):

1.  **System Prompt**: Load [AGENTS.md](AGENTS.md) as your system instruction. It defines the **RPI (Research → Plan → Implement)** loop.
2.  **Context Loading**: When the agent enters a specific phase (e.g., "Verification"), manually load the relevant `SKILL.md` (e.g., `skills/lint-hunter/SKILL.md`).

---

## 🤝 How to Contribute

We welcome new skills! Follow the **Triad Pattern**:

1.  **Create Directory**: `skills/<your-skill-name>/`.
2.  **Create Driver**: Add `SKILL.md` with YAML frontmatter:
    ```yaml
    ---
    name: My Skill
    description: What it does.
    rpi_phase: Implementation
    trigger:
      - "keyword1"
      - "keyword2"
    capabilities:
      - capability 1
    ---
    ```
3.  **Add Tools**: Put executable scripts in `skills/<your-skill-name>/scripts/`.
4.  **Add Context**: Put documentation in `skills/<your-skill-name>/references/`.
5.  **Generate Docs**: Run `make doc` to update `AGENTS.md`.

---

## 🔮 Roadmap (In Progress / TBD)

- [ ] **Multi-Agent Beast Mode**: Chaining multiple skills in a single "Beast Mode" loop without human intervention.
- [ ] **New Skill**: `Test Architect` (Refactoring & property-based testing).
- [ ] **New Skill**: `Crates.io Scout` (Dependency analysis).
- [ ] **Automated CI**: GitHub Action to run `make verify` on PRs.
- [ ] **Vector Embeddings**: Indexing `references/` for semantic search retrieval.

---

## 📜 License

MIT
