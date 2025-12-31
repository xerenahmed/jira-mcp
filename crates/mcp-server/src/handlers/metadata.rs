use anyhow::Result;
use rmcp::model::CallToolResult;

use super::super::context::JiraCtx;
use super::super::errors::log_err;
use super::super::models::{ListIssueTypesInput, MoveToBacklogInput};

pub async fn get_user_info_handler(ctx: &JiraCtx) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(target: "mcp", tool = "get_user_info");

    let info = ctx
        .client
        .get_myself(&ctx.auth)
        .await
        .map_err(|e| log_err("get_user_info", "jira_error", e.to_string()))?;

    Ok(CallToolResult::structured(
        serde_json::to_value(info).unwrap_or(serde_json::json!({})),
    ))
}

pub async fn list_issue_types_handler(
    input: ListIssueTypesInput,
    ctx: &JiraCtx,
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(target: "mcp", tool = "list_issue_types", project = ?input.project_key);

    let items = ctx
        .client
        .list_issue_types(input.project_key.as_deref(), &ctx.auth)
        .await
        .map_err(|e| log_err("list_issue_types", "jira_error", e.to_string()))?;

    Ok(CallToolResult::structured(
        serde_json::json!({"issue_types": items}),
    ))
}

pub async fn list_boards_handler(
    project_key: String,
    ctx: &JiraCtx,
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(target: "mcp", tool = "list_boards", project = %project_key);

    let boards = ctx
        .client
        .list_boards(&project_key, &ctx.auth)
        .await
        .map_err(|e| log_err("list_boards", "jira_error", e.to_string()))?;

    let total_count = boards.len();

    tracing::info!(
        target: "mcp",
        tool = "list_boards",
        project = %project_key,
        count = total_count,
        "Boards listed successfully"
    );

    Ok(CallToolResult::structured(
        serde_json::json!({
            "boards": boards,
            "total_count": total_count,
            "fully_paginated": true,
            "project_key": project_key
        }),
    ))
}

pub async fn move_to_backlog_handler(
    input: MoveToBacklogInput,
    ctx: &JiraCtx,
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(
        target: "mcp",
        tool = "move_to_backlog",
        issues = ?input.issue_keys
    );

    ctx.client
        .move_issues_to_backlog(&input.issue_keys, &ctx.auth)
        .await
        .map_err(|e| log_err("move_to_backlog", "jira_error", e.to_string()))?;

    tracing::info!(
        target: "mcp",
        tool = "move_to_backlog",
        issues = ?input.issue_keys,
        "Issues moved to backlog successfully"
    );

    Ok(CallToolResult::structured(
        serde_json::json!({
            "success": true,
            "message": format!("Successfully moved {} issue(s) to the backlog", input.issue_keys.len()),
            "moved_issues": input.issue_keys
        }),
    ))
}
