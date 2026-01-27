use crate::auth::Auth;
use crate::client::JiraClient;
use anyhow::Result;

/// Jira configuration - all fields required.
/// Constructed from CLI args or environment variables.
#[derive(Debug, Clone)]
pub struct JiraConfig {
    pub jira_base_url: String,
    pub username: String,
    pub token: String,
}

impl JiraConfig {
    pub fn new(jira_base_url: String, username: String, token: String) -> Self {
        Self {
            jira_base_url,
            username,
            token,
        }
    }

    pub fn create_auth(&self) -> Auth {
        Auth {
            username: self.username.clone(),
            token: self.token.clone(),
        }
    }

    pub fn create_client(&self) -> Result<JiraClient> {
        tracing::info!(base_url = %self.jira_base_url, "Creating Jira client");
        let auth = self.create_auth();
        JiraClient::new(&self.jira_base_url, auth)
    }
}
