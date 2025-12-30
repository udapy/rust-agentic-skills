# Pest: Grammar Cheat Sheet

### PEG Syntax Constraints

1.  **Syntactic vs Lexical**:
    - Atomic rules (`rule @{ ... }`) generally do NOT consume internal whitespace.
    - Compound rules (`rule = { ... }`) DO consume whitespace implicitly if `WHITESPACE` is defined.
2.  **Special Rules**:
    - `WHITESPACE = _{ " " | "\t" | "\n" }` (Underscore `_` makes it silent).
    - `COMMENT = _{ "//" ~ (!NEWLINE ~ ANY)* }`
3.  **Anchors**:
    - Always start the top-level rule with `SOI` (Start of Input) and end with `EOI`.
    - _Example_: `file = { SOI ~ (stmt)* ~ EOI }`
4.  **Greediness**: - `*` and `+` are eager. - Ordered choice `|` is first-match-wins. Put specific matches first (e.g., `"<=" | "<"`).
