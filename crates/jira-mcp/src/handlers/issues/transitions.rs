use rmcp::model::CallToolResult;

use crate::context::JiraCtx;
use crate::errors::{ResultMcpExt};
use crate::errors::suggestions::get_transition_suggestions;
use crate::models::{GetTransitionsInput, TransitionIssueInput, AssignIssueInput};
use crate::error_ctx;
use crate::handlers::error_utils::extract_error_message;

pub async fn get_transitions_handler(
    input: GetTransitionsInput,
    ctx: &JiraCtx,
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(
        target: "mcp",
        tool = "get_transitions",
        issue_key = %input.issue_key,
        expand = ?input.expand,
        "Getting transitions for issue"
    );

    let issue_key = input.issue_key.clone();
    let result = ctx
        .client
        .get_transitions(
            &input.issue_key,
            input.expand.as_deref(),
            &ctx.auth,
        )
        .await
        .mcp_context(
            error_ctx!("get_transitions", "get transitions")
                .with_metadata("issue_key", input.issue_key.clone())
                .with_suggestions(move |status| get_transition_suggestions(&issue_key, status))
        )?;

    let transitions: Vec<serde_json::Value> = result
        .get("transitions")
        .and_then(|t| t.as_array())
        .map(|arr| {
            arr.iter()
                .map(|t| {
                    let id = t.get("id").and_then(|v| v.as_str()).unwrap_or("");
                    let name = t.get("name").and_then(|v| v.as_str()).unwrap_or("");
                    let to_name = t
                        .get("to")
                        .and_then(|to| to.get("name"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    let category = t
                        .get("to")
                        .and_then(|to| to.get("statusCategory"))
                        .and_then(|sc| sc.get("name"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");

                    serde_json::json!({
                        "id": id,
                        "name": name,
                        "to": to_name,
                        "category": category
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    tracing::info!(
        target: "mcp",
        tool = "get_transitions",
        issue_key = %input.issue_key,
        count = transitions.len(),
        "Transitions retrieved successfully"
    );

    Ok(CallToolResult::structured(serde_json::json!({
        "issue_key": input.issue_key,
        "transitions": transitions
    })))
}

pub async fn transition_issue_handler(
    input: TransitionIssueInput,
    ctx: &JiraCtx,
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(
        target: "mcp",
        tool = "transition_issue",
        issue_key = %input.issue_key,
        transition_id = %input.transition_id,
        "Transitioning issue"
    );

    let issue_key = input.issue_key.clone();
    ctx.client
        .transition_issue(
            &input.issue_key,
            &input.transition_id,
            input.fields.as_ref(),
            input.comment.as_deref(),
            &ctx.auth,
        )
        .await
        .mcp_context(
            error_ctx!("transition_issue", "transition issue")
                .with_metadata("issue_key", input.issue_key.clone())
                .with_metadata("transition_id", input.transition_id.clone())
                .with_suggestions(move |status| get_transition_suggestions(&issue_key, status))
        )?;

    tracing::info!(
        target: "mcp",
        tool = "transition_issue",
        issue_key = %input.issue_key,
        transition_id = %input.transition_id,
        "Issue transitioned successfully"
    );

    Ok(CallToolResult::structured(serde_json::json!({
        "issue_key": input.issue_key,
        "transition_id": input.transition_id,
        "success": true,
        "message": format!("Issue {} transitioned successfully", input.issue_key)
    })))
}

pub async fn assign_issue_handler(
    input: AssignIssueInput,
    ctx: &JiraCtx,
) -> Result<CallToolResult, rmcp::ErrorData> {
    let action = if input.account_id.is_some() { "Assigning" } else { "Unassigning" };
    tracing::info!(
        target: "mcp",
        tool = "assign_issue",
        issue_key = %input.issue_key,
        account_id = ?input.account_id,
        "{} user for issue", action
    );

    ctx.client
        .assign_issue(
            &input.issue_key,
            input.account_id.as_deref(),
            &ctx.auth,
        )
        .await
        .map_err(|e| {
            tracing::error!(
                target: "mcp",
                tool = "assign_issue",
                error = %e,
                issue_key = %input.issue_key,
                "Failed to assign issue"
            );

            if let Some(jira_client::error::JiraError::ApiError { status_code, response }) = e.downcast_ref::<jira_client::error::JiraError>() {
                let error_message = extract_error_message(response);

                return rmcp::ErrorData::internal_error(
                    format!("Jira API Error ({}): {}", status_code, error_message),
                    Some(serde_json::json!({
                        "issue_key": input.issue_key,
                        "account_id": input.account_id,
                        "status_code": status_code,
                        "jira_response": response
                    })),
                );
            }

            rmcp::ErrorData::internal_error(
                format!("Failed to assign issue {}: {}", input.issue_key, e),
                None
            )
        })?;

    let status = if input.account_id.is_some() { "assigned" } else { "unassigned" };
    tracing::info!(
        target: "mcp",
        tool = "assign_issue",
        issue_key = %input.issue_key,
        status = status,
        "Issue {} successfully", status
    );

    Ok(CallToolResult::structured(serde_json::json!({
        "success": true,
        "issue_key": input.issue_key,
        "status": status,
        "account_id": input.account_id
    })))
}
