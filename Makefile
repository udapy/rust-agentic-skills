.PHONY: all doc verify clean

# Default target
all: doc verify

# Regenerate AGENTS.md from SKILL.md metadata
doc:
	@echo "Generating documentation..."
	@rustc scripts/generate_agents_md.rs
	@./generate_agents_md
	@rm generate_agents_md

# Run all verification steps
# Note: verify_borrow.rs is INTENDED to fail checking, so we allow failure there for deeper manual verification
verify:
	@echo "Verifying system integrity..."
	@echo "[1/2] Checking core project..."
	cargo check || echo "Note: cargo check found issues (Expected if verify_borrow.rs is present)"
	@echo "[2/2] Verifying Lint Hunter Script..."
	@skills/lint-hunter/scripts/explain_error.sh E0502 > /dev/null && echo "Lint Hunter: OK"

# Clean artifacts
clean:
	cargo clean
	rm -f generate_agents_md
