default: fmt

fmt:
    cargo fmt --all

lint:
    cargo clippy --workspace --all-targets -D warnings

build:
    cargo build --workspace

run-mcp:
    cargo run -p jira-mcp

run-cli:
    cargo run -p jira-assistant-cli

