use jira_client::{client::JiraClient, auth::Auth as JiraAuth, config::JiraConfig};

use super::errors::log_err;

#[derive(Clone)]
pub struct JiraCtx {
    pub auth: JiraAuth,
    pub client: JiraClient,
}
pub fn jira_ctx() -> Result<JiraCtx, rmcp::ErrorData> {
    tracing::info!(target: "mcp", "Loading Jira configuration");
    let cfg = JiraConfig::load_default().map_err(|e| {
        tracing::error!(target: "mcp", error = %e, "Failed to load Jira config");
        log_err("ctx", "config_error", e.to_string())
    })?;
    tracing::info!(target: "mcp", base_url = %cfg.jira_base_url, auth_method = %cfg.auth.method, "Config loaded");
    let auth = cfg.create_auth();
    let client = JiraClient::new(&cfg.jira_base_url, auth.clone()).map_err(|e| {
        tracing::error!(target: "mcp", error = %e, "Failed to create Jira client");
        log_err("ctx", "client_error", e.to_string())
    })?;
    tracing::info!(target: "mcp", "Jira context initialized");
    Ok(JiraCtx { auth, client })
}
