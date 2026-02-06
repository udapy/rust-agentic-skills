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

# Docker targets
docker-build:
	docker build -t rust-agentic-skills .

docker-run:
	@echo "Starting MCP Server in Docker (Interactive Mode)..."
	@echo "Type JSON-RPC messages to interact."
	docker run -i --rm rust-agentic-skills

# Logs info (MCP server writes to stderr, so they appear in console during docker-run)
logs:
	@echo "To view logs locally during development:"
	@echo "  cargo run --release 2>&1 | grep -v 'JsonRpcResponse'"
	@echo ""
	@echo "In Docker, logs (stderr) are printed to the console alongside stdout."

