use jira_client::{auth::Auth, client::JiraClient, config::JiraConfig};

use super::errors::log_err;

#[derive(Clone)]
pub struct JiraCtx {
    pub auth: Auth,
    pub client: JiraClient,
}

impl JiraCtx {
    pub fn from_config(config: &JiraConfig) -> Result<Self, rmcp::ErrorData> {
        tracing::info!(target: "mcp", base_url = %config.jira_base_url, "Creating Jira context");

        let auth = config.create_auth();
        let client = JiraClient::new(&config.jira_base_url, auth.clone()).map_err(|e| {
            tracing::error!(target: "mcp", error = %e, "Failed to create Jira client");
            log_err("ctx", "client_error", e.to_string())
        })?;

        tracing::info!(target: "mcp", "Jira context initialized");
        Ok(JiraCtx { auth, client })
    }
}
