use rmcp::model::CallToolResult;

use crate::context::JiraCtx;
use crate::errors::ResultMcpExt;
use crate::errors::suggestions::get_watcher_suggestions;
use crate::models::{AddWatcherInput, RemoveWatcherInput, GetWatchersInput};
use crate::error_ctx;
use crate::handlers::error_utils::extract_error_message;

pub async fn add_watcher_handler(
    input: AddWatcherInput,
    ctx: &JiraCtx,
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(
        target: "mcp",
        tool = "add_watcher",
        issue_key = %input.issue_key,
        account_id = %input.account_id,
        "Adding watcher to issue"
    );

    let issue_key = input.issue_key.clone();
    ctx.client
        .add_watcher(&input.issue_key, &input.account_id, &ctx.auth)
        .await
        .mcp_context(
            error_ctx!("add_watcher", "add watcher")
                .with_metadata("issue_key", input.issue_key.clone())
                .with_metadata("account_id", input.account_id.clone())
                .with_suggestions(move |status| get_watcher_suggestions(&issue_key, status))
        )?;

    tracing::info!(
        target: "mcp",
        tool = "add_watcher",
        issue_key = %input.issue_key,
        account_id = %input.account_id,
        "Watcher added successfully"
    );

    Ok(CallToolResult::structured(
        serde_json::json!({
            "success": true,
            "issue_key": input.issue_key,
            "account_id": input.account_id,
            "message": format!("User {} added as watcher to issue {}", input.account_id, input.issue_key)
        }),
    ))
}

pub async fn remove_watcher_handler(
    input: RemoveWatcherInput,
    ctx: &JiraCtx,
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(
        target: "mcp",
        tool = "remove_watcher",
        issue_key = %input.issue_key,
        account_id = %input.account_id,
        "Removing watcher from issue"
    );

    let issue_key = input.issue_key.clone();
    ctx.client
        .remove_watcher(&input.issue_key, &input.account_id, &ctx.auth)
        .await
        .mcp_context(
            error_ctx!("remove_watcher", "remove watcher")
                .with_metadata("issue_key", input.issue_key.clone())
                .with_metadata("account_id", input.account_id.clone())
                .with_suggestions(move |status| get_watcher_suggestions(&issue_key, status))
        )?;

    tracing::info!(
        target: "mcp",
        tool = "remove_watcher",
        issue_key = %input.issue_key,
        account_id = %input.account_id,
        "Watcher removed successfully"
    );

    Ok(CallToolResult::structured(
        serde_json::json!({
            "success": true,
            "issue_key": input.issue_key,
            "account_id": input.account_id,
            "message": format!("Successfully removed watcher {} from issue {}", input.account_id, input.issue_key)
        }),
    ))
}

pub async fn get_watchers_handler(
    input: GetWatchersInput,
    ctx: &JiraCtx,
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(
        target: "mcp",
        tool = "get_watchers",
        issue_key = %input.issue_key,
        "Getting watchers for issue"
    );

    let response = ctx
        .client
        .get_watchers(&input.issue_key, &ctx.auth)
        .await
        .map_err(|e| {
            tracing::error!(
                target: "mcp",
                tool = "get_watchers",
                error = %e,
                issue_key = %input.issue_key,
                "Failed to get watchers"
            );

            if let Some(jira_client::error::JiraError::ApiError { status_code, response }) = e.downcast_ref::<jira_client::error::JiraError>() {
                let error_message = extract_error_message(response);

                return rmcp::ErrorData::internal_error(
                    format!("Jira API Error ({}): {}", status_code, error_message),
                    Some(serde_json::json!({
                        "issue_key": input.issue_key,
                        "status_code": status_code,
                        "jira_response": response
                    })),
                );
            }

            rmcp::ErrorData::internal_error(
                format!("Failed to get watchers for {}: {}", input.issue_key, e),
                None
            )
        })?;

    let watchers = response
        .get("watchers")
        .and_then(|w| w.as_array())
        .map(|arr| {
            arr.iter()
                .map(|w| {
                    serde_json::json!({
                        "accountId": w.get("accountId").and_then(|v| v.as_str()).unwrap_or(""),
                        "displayName": w.get("displayName").and_then(|v| v.as_str()).unwrap_or(""),
                        "active": w.get("active").and_then(|v| v.as_bool()).unwrap_or(false)
                    })
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let watch_count = response
        .get("watchCount")
        .and_then(|c| c.as_u64())
        .unwrap_or(0);

    let is_watching = response
        .get("isWatching")
        .and_then(|w| w.as_bool())
        .unwrap_or(false);

    tracing::info!(
        target: "mcp",
        tool = "get_watchers",
        issue_key = %input.issue_key,
        watcher_count = watchers.len(),
        "Got watchers successfully"
    );

    Ok(CallToolResult::structured(
        serde_json::json!({
            "issue_key": input.issue_key,
            "is_watching": is_watching,
            "watch_count": watch_count,
            "watchers": watchers
        }),
    ))
}
