# Jira MCP

## About
Developer-focused Model Context Protocol (MCP) server for Jira that communicates over stdio.

- Crates:
  - `crates/mcp-server`: MCP server exposing Jira tools
  - `crates/jira-client`: Jira REST client and config loader
  - `crates/core`: Shared models and helpers
- **31 tools** across 7 categories (see below)

## Available Tools

### Issue Management
| Tool | Description |
|------|-------------|
| `create_issue` | Create a Jira issue |
| `update_issue` | Update issue fields |
| `get_issue` | Get issue with full fields, name mapping, schema |
| `search_issues` | Search by JQL query |
| `assign_issue` | Assign/unassign user |
| `get_transitions` | Get available status transitions |
| `transition_issue` | Transition to new status |

### Comments
| Tool | Description |
|------|-------------|
| `add_comment` | Add comment (supports visibility restrictions) |
| `get_comments` | Get comments (ordered by created) |
| `update_comment` | Update existing comment |
| `delete_comment` | Delete comment |

### Labels
| Tool | Description |
|------|-------------|
| `list_labels` | List/search labels (use `query` for 200+ labels) |
| `add_label` | Add labels (accepts array) |
| `remove_label` | Remove labels (accepts array) |

### Watchers
| Tool | Description |
|------|-------------|
| `get_watchers` | Get all watchers |
| `add_watcher` | Add watcher |
| `remove_watcher` | Remove watcher |

### Issue Links
| Tool | Description |
|------|-------------|
| `list_link_types` | List available link types (Blocks, Duplicates, etc.) |
| `link_issues` | Create link between issues |
| `delete_issue_link` | Delete issue link |

### Sprint Management
| Tool | Description |
|------|-------------|
| `list_sprints` | List sprints for a board |
| `get_sprint` | Get sprint details |
| `move_to_sprint` | Move issues to sprint |
| `move_to_backlog` | Move issues to backlog |

### Metadata & Users
| Tool | Description |
|------|-------------|
| `list_projects` | List accessible projects |
| `list_boards` | List boards for project |
| `list_issue_types` | List issue types |
| `list_fields` | List fields for project/issue type |
| `get_field_details` | Get field schema and allowed values |
| `get_user_info` | Get current user info |
| `search_users` | Search users by name/email |

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
