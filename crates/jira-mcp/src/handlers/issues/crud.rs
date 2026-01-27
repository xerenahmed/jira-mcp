use std::collections::HashSet;

use rmcp::model::CallToolResult;

use crate::context::JiraCtx;
use crate::errors::log_err;
use crate::handlers::error_utils::{extract_error_message, get_jql_suggestions, get_create_suggestions, get_update_suggestions};
use crate::models::{SearchIssuesInput, GetIssueInput, CreateIssueInput, CreateIssueResult, UpdateIssueInput, UpdateIssueResult};
use crate::services::CoreService;

pub async fn create_issue_handler(
    input: CreateIssueInput,
    ctx: &JiraCtx,
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(
        target: "mcp",
        tool = "create_issue",
        "Creating Jira issue"
    );

    let svc = CoreService::new(ctx);

    let res = svc
        .create_issue(input.clone())
        .await
        .map_err(|e| {
            tracing::error!(
                target: "mcp",
                tool = "create_issue",
                error = %e,
                "Failed to create issue"
            );

            if let Some(jira_client::error::JiraError::ApiError { status_code, response }) = e.downcast_ref::<jira_client::error::JiraError>() {
                let error_message = extract_error_message(response);
                let project_key = input.fields.get("project")
                    .and_then(|p| p.get("key"))
                    .and_then(|k| k.as_str())
                    .map(|s| s.to_string());
                let issue_type = input.fields.get("issuetype")
                    .and_then(|it| it.get("name"))
                    .and_then(|n| n.as_str())
                    .map(|s| s.to_string());

                let suggestions = get_create_suggestions(&project_key, &issue_type, *status_code);
                let full_message = if suggestions.is_empty() {
                    format!("Jira API Error ({}): {}", status_code, error_message)
                } else {
                    format!("Jira API Error ({}): {}\n\nSuggestions:\n{}",
                        status_code,
                        error_message,
                        suggestions.iter().map(|s| format!("  - {}", s)).collect::<Vec<_>>().join("\n")
                    )
                };

                return rmcp::ErrorData::internal_error(
                    full_message,
                    Some(serde_json::json!({
                        "status_code": status_code,
                        "jira_response": response,
                        "suggestions": suggestions
                    })),
                );
            }
            rmcp::ErrorData::internal_error(
                format!("Failed to create issue: {}", e),
                None
            )
        })?;

    let CreateIssueResult::Created(payload) = res;
    tracing::info!(
        target: "mcp",
        tool = "create_issue",
        issue_key = %payload.issue_key,
        url = %payload.url,
        "Issue created successfully"
    );

    Ok(CallToolResult::structured(
        serde_json::to_value(payload).unwrap_or(serde_json::json!({})),
    ))
}

pub async fn update_issue_handler(
    input: UpdateIssueInput,
    ctx: &JiraCtx,
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(
        target: "mcp",
        tool = "update_issue",
        issue_key = %input.issue_key,
        "Updating Jira issue"
    );

    let svc = CoreService::new(ctx);

    let res = svc
        .update_issue(input.clone())
        .await
        .map_err(|e| {
            tracing::error!(
                target: "mcp",
                tool = "update_issue",
                error = %e,
                issue_key = %input.issue_key,
                "Failed to update issue"
            );

            if let Some(jira_client::error::JiraError::ApiError { status_code, response }) = e.downcast_ref::<jira_client::error::JiraError>() {
                let error_message = extract_error_message(response);
                let suggestions = get_update_suggestions(&input.issue_key, *status_code);

                let full_message = if suggestions.is_empty() {
                    format!("Jira API Error ({}): {}", status_code, error_message)
                } else {
                    format!("Jira API Error ({}): {}\n\nSuggestions:\n{}",
                        status_code,
                        error_message,
                        suggestions.iter().map(|s| format!("  - {}", s)).collect::<Vec<_>>().join("\n")
                    )
                };

                return rmcp::ErrorData::internal_error(
                    full_message,
                    Some(serde_json::json!({
                        "issue_key": input.issue_key,
                        "status_code": status_code,
                        "jira_response": response,
                        "suggestions": suggestions
                    })),
                );
            }

            rmcp::ErrorData::internal_error(
                format!("Failed to update issue {}: {}", input.issue_key, e),
                None
            )
        })?;

    let UpdateIssueResult::Updated(payload) = res;

    tracing::info!(
        target: "mcp",
        tool = "update_issue",
        issue_key = %payload.issue_key,
        updated_fields_count = payload.updated_fields.len(),
        "Issue updated successfully"
    );

    Ok(CallToolResult::structured(
        serde_json::to_value(payload).unwrap_or(serde_json::json!({})),
    ))
}

pub async fn search_issues_handler(
    input: SearchIssuesInput,
    ctx: &JiraCtx,
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(
        target: "mcp",
        tool = "search_issues",
        jql = %input.jql,
        limit = input.limit,
        start_at = ?input.start_at,
        fields = %input.fields,
        "Searching issues"
    );

    let results = ctx
        .client
        .search_issues(&input.jql, Some(&input.fields), input.limit, input.start_at, &ctx.auth)
        .await
        .map_err(|e| {
            tracing::error!(
                target: "mcp",
                tool = "search_issues",
                error = %e,
                jql = %input.jql,
                "Failed to search issues"
            );

            if let Some(jira_client::error::JiraError::ApiError { status_code, response }) = e.downcast_ref::<jira_client::error::JiraError>() {
                let error_message = extract_error_message(response);
                let suggestions = get_jql_suggestions(&input.jql, *status_code);

                let full_message = if suggestions.is_empty() {
                    format!("Jira Search Error ({}): {}", status_code, error_message)
                } else {
                    format!("Jira Search Error ({}): {}\n\nSuggestions:\n{}",
                        status_code,
                        error_message,
                        suggestions.iter().map(|s| format!("  - {}", s)).collect::<Vec<_>>().join("\n")
                    )
                };

                return rmcp::ErrorData::internal_error(
                    full_message,
                    Some(serde_json::json!({
                        "jql": input.jql,
                        "status_code": status_code,
                        "jira_response": response,
                        "suggestions": suggestions
                    })),
                );
            }

            rmcp::ErrorData::internal_error(
                format!("Failed to search issues: {}", e),
                None
            )
        })?;

    tracing::info!(
        target: "mcp",
        tool = "search_issues",
        count = results.len(),
        "Search completed"
    );

    Ok(CallToolResult::structured(
        serde_json::json!({ "results": results }),
    ))
}

pub async fn get_issue_handler(
    input: GetIssueInput,
    ctx: &JiraCtx,
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(target: "mcp", tool = "get_issue", key = %input.key, board_id = %input.board_id);
    let mut detail = ctx
        .client
        .get_issue_detail(&input.key, &ctx.auth)
        .await
        .map_err(|e| {
            tracing::error!(
                target: "mcp",
                tool = "get_issue",
                error = %e,
                issue_key = %input.key,
                board_id = %input.board_id,
                "Failed to get issue"
            );

            if let Some(jira_client::error::JiraError::ApiError { status_code, response }) = e.downcast_ref::<jira_client::error::JiraError>() {
                let error_message = extract_error_message(response);

                return rmcp::ErrorData::internal_error(
                    format!("Jira API Error ({}): {}", status_code, error_message),
                    Some(serde_json::json!({
                        "issue_key": input.key,
                        "board_id": input.board_id,
                        "status_code": status_code,
                        "jira_response": response
                    })),
                );
            }
            rmcp::ErrorData::internal_error(
                format!("Failed to get issue {}: {}", input.key, e),
                None
            )
        })?;
    let keys = crate::board_utils::compute_board_field_keys(ctx, &detail, input.board_id)
        .await
        .map_err(|e| {
            tracing::error!(
                target: "mcp",
                tool = "get_issue",
                error = %e,
                issue_key = %input.key,
                board_id = %input.board_id,
                "Failed to compute board field keys"
            );
            log_err("get_issue", "board_utils_error", e.to_string())
        })?;
    filter_issue_fields_for_board(&mut detail, keys);
    Ok(CallToolResult::structured(
        serde_json::to_value(detail).unwrap_or(serde_json::json!({})),
    ))
}

fn filter_issue_fields_for_board(
    detail: &mut jira_client::models::IssueDetail,
    keys: HashSet<String>,
) {
    let present: HashSet<String> = detail
        .fields
        .as_object()
        .map(|o| o.keys().cloned().collect())
        .unwrap_or_default();
    let kept: HashSet<String> = keys.intersection(&present).cloned().collect();

    if let Some(obj) = detail.fields.as_object_mut() {
        let to_remove: Vec<String> = obj.keys().filter(|k| !kept.contains(*k)).cloned().collect();
        for k in to_remove {
            obj.remove(&k);
        }
    }
}
