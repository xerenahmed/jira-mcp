use anyhow::Result;

use crate::context::JiraCtx;
use crate::models::{
    CreateIssueInput, CreateIssueResult, CreatedIssue,
    UpdateIssueInput, UpdateIssueResult, UpdatedIssue,
};
use crate::utils::field_processing::process_jira_fields;

pub struct CoreService<'a> {
    ctx: &'a JiraCtx,
}

impl<'a> CoreService<'a> {
    pub fn new(ctx: &'a JiraCtx) -> Self {
        CoreService { ctx }
    }

    pub async fn create_issue(&self, input: CreateIssueInput) -> Result<CreateIssueResult> {
        tracing::info!(
            target: "core",
            op = "create_issue",
            "Starting issue creation"
        );

        let processed_fields = process_jira_fields(&input.fields);
        let create_payload = serde_json::json!({ "fields": processed_fields });

        tracing::info!(target: "core", create_payload = ?create_payload, "Sending create request to Jira");

        let (key, url) = self.ctx.client.create_issue(&create_payload, &self.ctx.auth).await?;

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

        let processed_fields = process_jira_fields(&input.fields);
        let updated_fields: Vec<String> = processed_fields.keys().cloned().collect();

        let update_payload = serde_json::json!({
            "fields": processed_fields
        });

        tracing::info!(target: "core", update_payload = ?update_payload, "Sending update to Jira");

        self.ctx.client.update_issue(&input.issue_key, &update_payload, &self.ctx.auth).await?;

        let issue_url = self.ctx.client.base_url().join(&format!("/browse/{}", input.issue_key))?.to_string();

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
