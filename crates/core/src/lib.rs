use anyhow::{anyhow, Result};
use jira_client::{client::JiraClient, auth::Auth as JiraAuth, config::JiraConfig};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
mod utils;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UpdateIssueInput {
    pub issue_key: String,
    #[schemars(with = "serde_json::Value")]
    pub fields: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UpdatedIssue {
    pub issue_key: String,
    pub url: String,
    pub updated_fields: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "result", rename_all = "snake_case")]
pub enum UpdateIssueResult {
    Updated(UpdatedIssue),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateIssueInput {
    #[schemars(with = "serde_json::Value")]
    pub fields: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreatedIssue {
    pub issue_key: String,
    pub url: String,
    pub actions: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "result", rename_all = "snake_case")]
pub enum CreateIssueResult {
    Created(CreatedIssue),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FieldDef {
    pub id: String,
    pub name: String,
    pub required: bool,
    pub schema: serde_json::Value,
    #[serde(default, rename = "allowed_values")]
    pub allowed_values: serde_json::Value,
}

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

        let processed_fields = utils::process_jira_fields(&input.fields);
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
}

impl CoreService {

    pub async fn update_issue(&self, input: UpdateIssueInput) -> Result<UpdateIssueResult> {
        tracing::info!(target: "core", op = "update_issue", issue_key = %input.issue_key, fields = ?input.fields);

        if self.jira.is_none() || self.auth.is_none() || self.cfg.is_none() {
            return Err(anyhow!("configuration missing; configure Jira credentials"));
        }

        let (jira, auth) = (self.jira.as_ref().unwrap(), self.auth.as_ref().unwrap());

        let processed_fields = utils::process_jira_fields(&input.fields);
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

pub fn fields_from_createmeta(
    meta: &Value,
    project_key: Option<&str>,
    issue_type: Option<&str>,
) -> Vec<FieldDef> {
    let mut out = Vec::new();

    if let Some(projects) = meta.get("projects").and_then(|v| v.as_array()) {
        'p: for p in projects {
            if let Some(pk) = project_key {
                if p.get("key").and_then(|v| v.as_str()) != Some(pk) {
                    continue 'p;
                }
            }

            if let Some(its) = p.get("issuetypes").and_then(|v| v.as_array()) {
                for it in its {
                    if let Some(itn) = issue_type {
                        if it.get("name").and_then(|v| v.as_str()) != Some(itn) {
                            continue;
                        }
                    }

                    if let Some(fields) = it.get("fields").and_then(|v| v.as_object()) {
                        for (fid, fdef) in fields.iter() {
                            let name = fdef
                                .get("name")
                                .and_then(|v| v.as_str())
                                .unwrap_or(fid)
                                .to_string();

                            let required = fdef
                                .get("required")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false);

                            let schema =
                                fdef.get("schema").cloned().unwrap_or(serde_json::json!({}));

                            let allowed_values = fdef
                                .get("allowedValues")
                                .cloned()
                                .unwrap_or(serde_json::json!([]));

                            out.push(FieldDef {
                                id: fid.clone(),
                                name,
                                required,
                                schema,
                                allowed_values,
                            });
                        }
                    }
                }
            }
        }
    }

    out
}
