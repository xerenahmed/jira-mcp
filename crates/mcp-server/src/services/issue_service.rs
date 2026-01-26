use anyhow::{anyhow, Result};
use jira_client::{client::JiraClient, auth::Auth as JiraAuth, config::JiraConfig};

use crate::models::{
    CreateIssueInput, CreateIssueResult, CreatedIssue,
    UpdateIssueInput, UpdateIssueResult, UpdatedIssue,
};
use crate::utils::field_processing::process_jira_fields;

pub struct CoreService {
    pub jira: Option<JiraClient>,
    pub auth: Option<JiraAuth>,
    pub cfg: Option<JiraConfig>,
}

impl Default for CoreService {
    fn default() -> Self {
        Self::new()
    }
}

impl CoreService {
    pub fn new() -> Self {
        Self::from_config(JiraConfig::load_default().ok())
    }

    pub fn from_config(cfg: Option<JiraConfig>) -> Self {
        let (jira, auth, cfg) = if let Some(cfg) = cfg {
            let auth = cfg.create_auth();
            let jira = JiraClient::new(&cfg.jira_base_url, auth.clone())
                .expect("valid jira base url in config");

            (Some(jira), Some(auth), Some(cfg))
        } else {
            (None, None, None)
        };

        CoreService {
            jira,
            auth,
            cfg,
        }
    }

    pub async fn create_issue(&self, input: CreateIssueInput) -> Result<CreateIssueResult> {
        tracing::info!(
            target: "core",
            op = "create_issue",
            "Starting issue creation"
        );

        if self.jira.is_none() || self.auth.is_none() || self.cfg.is_none() {
            tracing::error!(target: "core", "Configuration missing");

            return Err(anyhow!("configuration missing; configure Jira credentials"));
        }

        let (jira, auth) = (self.jira.as_ref().unwrap(), self.auth.as_ref().unwrap());

        let processed_fields = process_jira_fields(&input.fields);
        let create_payload = serde_json::json!({ "fields": processed_fields });

        tracing::info!(target: "core", create_payload = ?create_payload, "Sending create request to Jira");

        let (key, url) = jira.create_issue(&create_payload, auth).await?;

        let created = CreatedIssue {
            issue_key: key.clone(),
            url: url.clone(),
            actions: vec!["created".into()],
            warnings: vec![],
        };

        tracing::info!(target: "core", issue_key = %key, url = %url, "Issue created successfully");
        Ok(CreateIssueResult::Created(created))
    }

    pub async fn update_issue(&self, input: UpdateIssueInput) -> Result<UpdateIssueResult> {
        tracing::info!(target: "core", op = "update_issue", issue_key = %input.issue_key, fields = ?input.fields);

        if self.jira.is_none() || self.auth.is_none() || self.cfg.is_none() {
            return Err(anyhow!("configuration missing; configure Jira credentials"));
        }

        let (jira, auth) = (self.jira.as_ref().unwrap(), self.auth.as_ref().unwrap());

        let processed_fields = process_jira_fields(&input.fields);
        let updated_fields: Vec<String> = processed_fields.keys().cloned().collect();

        let update_payload = serde_json::json!({
            "fields": processed_fields
        });

        tracing::info!(target: "core", update_payload = ?update_payload, "Sending update to Jira");

        jira.update_issue(&input.issue_key, &update_payload, auth).await?;

        let issue_url = jira.base_url().join(&format!("/browse/{}", input.issue_key))?.to_string();

        let updated = UpdatedIssue {
            issue_key: input.issue_key.clone(),
            url: issue_url,
            updated_fields,
            warnings: vec![],
        };

        tracing::info!(target: "core", issue_key = %input.issue_key, "Issue updated successfully");
        Ok(UpdateIssueResult::Updated(updated))
    }
}
