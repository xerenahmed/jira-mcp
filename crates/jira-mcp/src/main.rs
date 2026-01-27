use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::Parser;
use directories::ProjectDirs;
use jira_client::config::JiraConfig;
use jira_mcp::server::serve_stdio;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};

#[derive(Parser)]
#[command(name = "jira-mcp")]
#[command(about = "Jira MCP Server - Model Context Protocol server for Jira")]
struct Args {
    /// Jira base URL (e.g., https://your-domain.atlassian.net)
    #[arg(long, env = "JIRA_BASE_URL")]
    jira_url: String,

    /// Jira username/email
    #[arg(long, env = "JIRA_USERNAME")]
    username: String,

    /// Jira API token
    #[arg(long, env = "JIRA_TOKEN")]
    token: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = init_tracing();

    let args = Args::parse();

    let config = JiraConfig::new(args.jira_url, args.username, args.token);

    tracing::info!(base_url = %config.jira_base_url, "Starting Jira MCP server");

    serve_stdio(config).await
}

fn init_tracing() -> tracing_appender::non_blocking::WorkerGuard {
    let log_path = get_log_dir();

    if fs::create_dir_all(&log_path).is_err() {
        eprintln!("Warning: Failed to create log directory at {}, falling back to temp directory", log_path.display());
        let fallback_path = std::env::temp_dir().join("jira-mcp-logs");
        if let Err(e) = fs::create_dir_all(&fallback_path) {
            panic!("Failed to create log directory at fallback location {}: {}", fallback_path.display(), e);
        }
        setup_logging(&fallback_path)
    } else {
        setup_logging(&log_path)
    }
}

fn setup_logging(log_path: &Path) -> tracing_appender::non_blocking::WorkerGuard {
    let log_file = log_path.join("jira-mcp.log");

    let file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)
        .expect("failed to open log file");

    let (non_blocking, guard) = tracing_appender::non_blocking(file);

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("debug"));

    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking)
        .with_target(true)
        .with_line_number(true)
        .with_thread_ids(true);

    Registry::default()
        .with(env_filter)
        .with(file_layer)
        .init();

    eprintln!("Logging to: {}", log_file.display());

    guard
}

fn get_log_dir() -> PathBuf {
    if let Ok(log_dir) = std::env::var("JIRA_MCP_LOG_DIR") {
        return PathBuf::from(log_dir);
    }

    if let Some(dirs) = ProjectDirs::from("", "", "jira-mcp") {
        dirs.data_dir().join("logs")
    } else {
        std::env::temp_dir().join("jira-mcp-logs")
    }
}
