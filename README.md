# Jira MCP

## About
Developer-focused Model Context Protocol (MCP) server for Jira that communicates over stdio.

- Crates:
  - `crates/mcp-server`: MCP server exposing Jira tools
  - `crates/jira-client`: Jira REST client
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

### Pre-built Releases

Download the latest binary for your platform from [GitHub Releases](https://github.com/xerenahmed/jira-mcp/releases):

| Platform | Archive |
|----------|---------|
| Linux x86_64 | `mcp-server-linux-x86_64.tar.gz` |
| macOS x86_64 (Intel) | `mcp-server-macos-x86_64.tar.gz` |
| macOS ARM64 (Apple Silicon) | `mcp-server-macos-arm64.tar.gz` |

```bash
# Example: Download and extract (Linux/macOS)
curl -L -o mcp-server.tar.gz https://github.com/xerenahmed/jira-mcp/releases/latest/download/mcp-server-macos-arm64.tar.gz
tar -xzf mcp-server.tar.gz
./mcp-server --help
```

**Note for macOS users:** If you see a security warning when first running the binary ("Apple could not verify..."), you can remove the quarantine attribute:

```bash
xattr -d com.apple.quarantine ./mcp-server
```

Or go to **System Settings > Privacy & Security > Security** and click **"Allow Anyway"** after attempting to run the binary.

### Building from Source

Requires Rust (stable). The repo pins the version via `rust-toolchain.toml`.

```bash
cargo build --release
```

The binary will be at `target/release/mcp-server`.

## Configuration

The server requires three parameters, provided via CLI arguments or environment variables:

| Parameter | CLI Flag | Environment Variable | Description |
|-----------|----------|---------------------|-------------|
| Jira URL | `--jira-url` | `JIRA_BASE_URL` | Your Jira instance URL (e.g., `https://your-domain.atlassian.net`) |
| Username | `--username` | `JIRA_USERNAME` | Your Jira username/email |
| API Token | `--token` | `JIRA_TOKEN` | Your [Jira API token](https://id.atlassian.com/manage-profile/security/api-tokens) |

CLI arguments take precedence over environment variables.

### Running Directly

```bash
# With CLI arguments
./mcp-server \
  --jira-url https://your-domain.atlassian.net \
  --username you@example.com \
  --token your-api-token

# With environment variables
JIRA_BASE_URL=https://your-domain.atlassian.net \
JIRA_USERNAME=you@example.com \
JIRA_TOKEN=your-api-token \
./mcp-server
```

## Adding to Claude

**Option 1: Using Claude CLI (Recommended)**
```bash
# Add to Claude using the CLI command
claude mcp add jira \
  -e JIRA_BASE_URL=https://your-domain.atlassian.net \
  -e JIRA_USERNAME=you@example.com \
  -e JIRA_TOKEN=your-api-token \
  -- /path/to/mcp-server
```

**Option 2: Manual Configuration**

Update your Claude config at `~/.claude.json`:
```json
{
  "mcpServers": {
    "jira": {
      "command": "/path/to/mcp-server",
      "env": {
        "JIRA_BASE_URL": "https://your-domain.atlassian.net",
        "JIRA_USERNAME": "you@example.com",
        "JIRA_TOKEN": "your-api-token"
      }
    }
  }
}
```

Restart Claude to load the server.

**Using Jira MCP with Claude**

Once installed, you can interact with Jira directly:
- "Create a new task in the ENG project"
- "Search for all open bugs assigned to me"
- "Update issue PROJ-123 with a comment"
- "List all fields available in our project"

## Contributing
- Format: `cargo fmt --all`
- Lint: `cargo clippy --workspace --all-targets -D warnings`
