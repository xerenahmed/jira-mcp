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

### Jira Configuration
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

### Adding to Claude

**Option 1: Using Claude CLI (Recommended)**
```bash
# Build the server first
cargo build --release

# Add to Claude using the CLI command
claude mcp add --transport stdio jira --env JIRA_MCP_CONFIG=~/.config/jira-mcp/config.toml -- /path/to/jira-mcp/target/release/mcp-server
```

**Option 2: Manual Configuration**

1. **Build the MCP server:**
```bash
cargo build --release
```

2. **Update your Claude config** at `~/.claude.json`:
```json
{
  "mcpServers": {
    "jira": {
      "command": "/path/to/jira-mcp/target/release/mcp-server",
      "env": {
        "JIRA_MCP_CONFIG": "~/.config/jira-mcp/config.toml"
      }
    }
  }
}
```

3. **Restart Claude** to load the Jira MCP server.

**Using Jira MCP with Claude**

Once installed, you can interact with Jira directly:
- "Create a new task in the ENG project"
- "Search for all open bugs assigned to me"
- "Update issue PROJ-123 with a comment"
- "List all fields available in our project"

## Contributing
- Format: `cargo fmt --all`
- Lint: `cargo clippy --workspace --all-targets -D warnings`
