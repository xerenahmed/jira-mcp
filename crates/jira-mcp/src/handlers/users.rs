use anyhow::Result;
use rmcp::model::CallToolResult;

use super::super::context::JiraCtx;
use super::super::errors::log_err;
use super::super::models::{SearchUsersInput};

pub async fn search_users_handler(
    input: SearchUsersInput,
    ctx: &JiraCtx,
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(
        target: "mcp",
        tool = "search_users",
        query = %input.query,
        max_results = input.max_results,
        project_key = ?input.project_key,
        issue_type = ?input.issue_type,
        assignable_only = input.assignable_only,
        "Searching Jira users"
    );

    let users = if input.assignable_only {
        ctx.client
            .search_assignable_users(
                &input.query,
                input.project_key.as_deref(),
                input.issue_type.as_deref(),
                Some(input.max_results),
                &ctx.auth,
            )
            .await
            .map_err(|e| {
                tracing::error!(
                    target: "mcp",
                    tool = "search_users",
                    error = %e,
                    query = %input.query,
                    "Failed to search assignable users"
                );
                log_err("search_users", "jira_error", e.to_string())
            })?
    } else {
        ctx.client
            .search_users(&input.query, Some(input.max_results), &ctx.auth)
            .await
            .map_err(|e| {
                tracing::error!(
                    target: "mcp",
                    tool = "search_users",
                    error = %e,
                    query = %input.query,
                    "Failed to search users"
                );
                log_err("search_users", "jira_error", e.to_string())
            })?
    };

    let user_results: Vec<serde_json::Value> = users
        .into_iter()
        .map(|user| {
            let mut user_json = serde_json::json!({
                "account_id": user.account_id,
                "display_name": user.display_name,
                "active": user.active,
                "account_type": user.account_type
            });

            if let Some(email) = user.email_address {
                user_json["email_address"] = serde_json::Value::String(email);
            }

            if let Some(timezone) = user.time_zone {
                user_json["time_zone"] = serde_json::Value::String(timezone);
            }

            if let Some(avatar_urls) = user.avatar_urls {
                user_json["avatar_urls"] = avatar_urls;
            }

            user_json
        })
        .collect();

    tracing::info!(
        target: "mcp",
        tool = "search_users",
        count = user_results.len(),
        query = %input.query,
        "Users found successfully"
    );

    Ok(CallToolResult::structured(
        serde_json::json!({ "users": user_results }),
    ))
}
