use rmcp::model::CallToolResult;

use crate::context::JiraCtx;
use crate::errors::ResultMcpExt;
use crate::errors::suggestions::get_label_suggestions;
use crate::models::{AddLabelInput, RemoveLabelInput};
use crate::error_ctx;
use crate::handlers::error_utils::extract_error_message;

pub async fn add_label_handler(
    input: AddLabelInput,
    ctx: &JiraCtx,
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(
        target: "mcp",
        tool = "add_label",
        issue_key = %input.issue_key,
        labels = ?input.labels,
        "Adding labels to issue"
    );

    let issue_key = input.issue_key.clone();
    ctx.client
        .add_labels(&input.issue_key, &input.labels, &ctx.auth)
        .await
        .mcp_context(
            error_ctx!("add_label", "add labels")
                .with_metadata("issue_key", input.issue_key.clone())
                .with_metadata("labels", serde_json::json!(input.labels))
                .with_suggestions(move |status| get_label_suggestions(&issue_key, status))
        )?;

    let count = input.labels.len();
    tracing::info!(
        target: "mcp",
        tool = "add_label",
        issue_key = %input.issue_key,
        count = count,
        "Labels added successfully"
    );

    Ok(CallToolResult::structured(
        serde_json::json!({
            "issue_key": input.issue_key,
            "labels_added": input.labels,
            "count": count
        }),
    ))
}

pub async fn remove_label_handler(
    input: RemoveLabelInput,
    ctx: &JiraCtx,
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(
        target: "mcp",
        tool = "remove_label",
        issue_key = %input.issue_key,
        labels = ?input.labels,
        "Removing labels from issue"
    );

    ctx.client
        .remove_labels(&input.issue_key, &input.labels, &ctx.auth)
        .await
        .map_err(|e| {
            tracing::error!(
                target: "mcp",
                tool = "remove_label",
                error = %e,
                issue_key = %input.issue_key,
                labels = ?input.labels,
                "Failed to remove labels"
            );

            if let Some(jira_client::error::JiraError::ApiError { status_code, response }) = e.downcast_ref::<jira_client::error::JiraError>() {
                let error_message = extract_error_message(response);

                return rmcp::ErrorData::internal_error(
                    format!("Jira API Error ({}): {}", status_code, error_message),
                    Some(serde_json::json!({
                        "issue_key": input.issue_key,
                        "labels": input.labels,
                        "status_code": status_code,
                        "jira_response": response
                    })),
                );
            }

            rmcp::ErrorData::internal_error(
                format!("Failed to remove labels from issue {}: {}", input.issue_key, e),
                None
            )
        })?;

    let count = input.labels.len();
    tracing::info!(
        target: "mcp",
        tool = "remove_label",
        issue_key = %input.issue_key,
        count = count,
        "Labels removed successfully"
    );

    Ok(CallToolResult::structured(
        serde_json::json!({
            "issue_key": input.issue_key,
            "labels_removed": input.labels,
            "count": count
        }),
    ))
}
