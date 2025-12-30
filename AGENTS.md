# The Guild Roster

Meet the specialized agents that power the environment. The Guild operates under the **Research -> Plan -> Implement (RPI)** methodology.

## 🔄 RPI Phase Alignment

Different agents shine in different phases of the Beast Mode loop:

| Phase | Primary Agent | Role |
| :--- | :--- | :--- |
| **I. Research** | **Agent Router** | Analyzing intent and identifying necessary contexts. |
| **II. Plan** | **Architect (System)** | The system prompt itself drives the creation of `implementation_plan.md`. |
| **III. Implement** | **Rust Core** | Generating the initial implementation. |
| **IV. Verify** | **Lint Hunter** | The gatekeeper. Activates on `cargo check` failure. |

## 🦀 Rust-Specific Agents (`.github/instructions/rust/`)

| Agent | Filename | Responsibility | Trigger |
| :--- | :--- | :--- | :--- |
| **Rust Core** | `rust.instructions.md` | General implementation, idiomatic patterns, safety. | "Implement", "Refactor", Default |
| **Lint Hunter** | `linthunter.instructions.md` | Compiler error resolution, lifetime tracing. | `cargo check` failure, "E0xxx" |
| **Pest Specialist** | `pest.instructions.md` | Parser generation using PEG grammars. | "Parse", "Grammar", ".pest" |
| **RON Specialist** | `ron.instructions.md` | Configuration & Serialization. | "Config", "Settings", ".ron" |

## 🛠️ General Agents (`.github/instructions/general/`)

| Agent | Filename | Responsibility | Trigger |
| :--- | :--- | :--- | :--- |
| **Agent Router** | `index.instructions.md` | Intent classification & delegation. | **Always Active (First Step)** |
| **Debug Helper** | `debug.instructions.md` | Logic error isolation (Binary search). | "Panic", "Wrong output", "Bug" |
| **Syntax Hunter** | `syntaxhunter.instructions.md` | Basic syntax repair (semicolons, braces). | "Syntax Error", "Unexpected token" |
| **Security Specialist**| `security-github.instructions.md` | Auditing for `unsafe` and secrets. | "Audit", "Review", "Safe?" |
