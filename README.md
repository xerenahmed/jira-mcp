# Jira MCP

## About
Developer-focused Model Context Protocol (MCP) server for Jira that communicates over stdio.

- Crates:
  - `crates/mcp-server`: MCP server exposing Jira tools
  - `crates/jira-client`: Jira REST client and config loader
  - `crates/core`: Shared models and helpers
- Tools: `create_issue`, `update_issue`, `search_issues`, `list_fields`, `get_field_details`, `get_issue`, `get_user_info`, `list_issue_types`, `list_boards`, `list_projects`, `search_users`

## Installing
- Requires Rust (stable). The repo pins via `rust-toolchain.toml`.
- Build: `cargo build --workspace`
- Run server (stdio): `cargo run -p mcp-server`

## Configuring
Create a `config.toml` and either set `JIRA_MCP_CONFIG` to its path or place it under a standard config dir (e.g. `~/.config/jira-mcp/config.toml`).

Example `config.toml`:
```
jira_base_url = "https://your-domain.atlassian.net"
default_project_key = "ENG"          # optional
default_issue_type = "Task"          # optional
board_default = 12345                 # optional

[auth]
method = "pat"                        # "pat" or "bearer"
username = "you@example.com"          # required for "pat"
token = "<jira_api_token_or_bearer>"  # do not commit secrets
```

## Contributing
- Format: `cargo fmt --all`
- Lint: `cargo clippy --workspace --all-targets -D warnings`
