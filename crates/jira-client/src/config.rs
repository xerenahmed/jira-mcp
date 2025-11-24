use std::env;
use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

use crate::auth::Auth;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JiraConfigAuth {
    pub method: String,
    pub username: Option<String>,
    pub token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JiraConfig {
    pub jira_base_url: String,
    #[serde(default)]
    pub default_project_key: Option<String>,
    #[serde(default)]
    pub default_issue_type: Option<String>,
    #[serde(default)]
    pub board_default: Option<u64>,
    pub auth: JiraConfigAuth,
}

impl JiraConfig {
    pub fn config_path() -> Result<PathBuf> {
        if let Ok(p) = env::var("JIRA_MCP_CONFIG") {
            let pb = PathBuf::from(p);

            return Ok(pb);
        }

        let mut candidates: Vec<PathBuf> = Vec::new();

        let base = env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")));

        if let Some(base) = base {
            candidates.push(base.join("jira-mcp").join("config.toml"));
        }

        if let Some(appdata) = env::var_os("APPDATA").map(PathBuf::from) {
            candidates.push(appdata.join("jira-mcp").join("config.toml"));
        }

        for c in &candidates {
            if c.exists() {
                return Ok(c.clone());
            }
        }

        candidates.into_iter().next().ok_or_else(|| {
            anyhow!("config path not found; set JIRA_MCP_CONFIG or create a config file")
        })
    }

    pub fn load_default() -> Result<Self> {
        let path = Self::config_path()?;

        let content =
            fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;

        let cfg: Self = toml::from_str(&content).with_context(|| "parsing config.toml")?;

        Ok(cfg)
    }

    pub fn create_auth(&self) -> Auth {
        let auth = match (
            &self.auth.method[..],
            self.auth.username.clone(),
            self.auth.token.clone(),
        ) {
            ("pat", Some(user), Some(token)) => {
                tracing::info!(username = %user, "Using PAT auth");
                Auth::Basic { username: user, token }
            }
            ("bearer", _, Some(token)) => {
                tracing::info!("Using Bearer auth");
                Auth::Bearer { token }
            }
            _ => {
                tracing::warn!("No valid auth configured");
                Auth::None
            }
        };
        auth
    }

    pub fn create_client(&self) -> Result<crate::client::JiraClient> {
        tracing::info!(base_url = %self.jira_base_url, auth_method = %self.auth.method, "Creating Jira client");

        let auth = self.create_auth();
        let client = crate::client::JiraClient::new(&self.jira_base_url, auth)?;

        Ok(client)
    }
}
