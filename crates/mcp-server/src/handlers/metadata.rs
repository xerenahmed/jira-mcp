use anyhow::Result;
use rmcp::model::CallToolResult;

use super::super::context::JiraCtx;
use super::super::errors::log_err;
use super::super::models::{ListIssueTypesInput, ListSprintsInput, MoveToSprintInput};

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

pub async fn list_sprints_handler(
    input: ListSprintsInput,
    ctx: &JiraCtx,
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(target: "mcp", tool = "list_sprints", board_id = input.board_id, state = ?input.state);

    let sprints = ctx
        .client
        .list_sprints(input.board_id, input.state.as_deref(), &ctx.auth)
        .await
        .map_err(|e| log_err("list_sprints", "jira_error", e.to_string()))?;

    let total_count = sprints.len();

    tracing::info!(
        target: "mcp",
        tool = "list_sprints",
        board_id = input.board_id,
        count = total_count,
        "Sprints listed successfully"
    );

    Ok(CallToolResult::structured(
        serde_json::json!({
            "sprints": sprints,
            "total_count": total_count,
            "board_id": input.board_id
        }),
    ))
}

pub async fn move_to_sprint_handler(
    input: MoveToSprintInput,
    ctx: &JiraCtx,
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(
        target: "mcp",
        tool = "move_to_sprint",
        sprint_id = input.sprint_id,
        issues = ?input.issue_keys
    );

    ctx.client
        .move_issues_to_sprint(input.sprint_id, &input.issue_keys, &ctx.auth)
        .await
        .map_err(|e| log_err("move_to_sprint", "jira_error", e.to_string()))?;

    let issue_count = input.issue_keys.len();

    tracing::info!(
        target: "mcp",
        tool = "move_to_sprint",
        sprint_id = input.sprint_id,
        count = issue_count,
        "Issues moved to sprint successfully"
    );

    Ok(CallToolResult::structured(
        serde_json::json!({
            "success": true,
            "sprint_id": input.sprint_id,
            "moved_issues": input.issue_keys,
            "count": issue_count
        }),
    ))
}
