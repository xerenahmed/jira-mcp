use rmcp::model::CallToolResult;

use crate::context::JiraCtx;
use crate::errors::ResultMcpExt;
use crate::errors::suggestions::get_comment_suggestions;
use crate::models::{AddCommentInput, GetCommentsInput, UpdateCommentInput, DeleteCommentInput};
use crate::error_ctx;
use crate::handlers::error_utils::extract_error_message;
use jira_client::utils::adf_collect_text;

pub async fn add_comment_handler(
    input: AddCommentInput,
    ctx: &JiraCtx,
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(
        target: "mcp",
        tool = "add_comment",
        issue_key = %input.issue_key,
        "Adding comment to Jira issue"
    );

    let visibility = match (input.visibility_type, input.visibility_value) {
        (Some(vis_type), Some(vis_value)) => Some(jira_client::api::issues::CommentVisibility {
            visibility_type: vis_type,
            value: vis_value,
        }),
        _ => None,
    };

    let issue_key = input.issue_key.clone();
    let response = ctx
        .client
        .add_comment(&input.issue_key, &input.body, visibility, &ctx.auth)
        .await
        .mcp_context(
            error_ctx!("add_comment", "add comment")
                .with_metadata("issue_key", input.issue_key.clone())
                .with_suggestions(move |status| get_comment_suggestions(&issue_key, status))
        )?;

    let comment_id = response.get("id").and_then(|v| v.as_str()).unwrap_or("");
    let author_name = response
        .get("author")
        .and_then(|a| a.get("displayName"))
        .and_then(|n| n.as_str())
        .unwrap_or("");
    let created = response.get("created").and_then(|v| v.as_str()).unwrap_or("");

    tracing::info!(
        target: "mcp",
        tool = "add_comment",
        issue_key = %input.issue_key,
        comment_id = %comment_id,
        "Comment added successfully"
    );

    Ok(CallToolResult::structured(
        serde_json::json!({
            "id": comment_id,
            "issue_key": input.issue_key,
            "author": author_name,
            "created": created
        }),
    ))
}

fn adf_to_plain_text(adf: &serde_json::Value) -> String {
    let mut result = String::new();
    adf_collect_text(adf, &mut result);
    result.trim().to_string()
}

pub async fn get_comments_handler(
    input: GetCommentsInput,
    ctx: &JiraCtx,
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(
        target: "mcp",
        tool = "get_comments",
        issue_key = %input.issue_key,
        max_results = ?input.max_results,
        order_by = ?input.order_by,
        "Getting comments for issue"
    );

    let response = ctx
        .client
        .get_comments(
            &input.issue_key,
            input.max_results,
            input.order_by.as_deref(),
            &ctx.auth,
        )
        .await
        .map_err(|e| {
            tracing::error!(
                target: "mcp",
                tool = "get_comments",
                error = %e,
                issue_key = %input.issue_key,
                "Failed to get comments"
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
                format!("Failed to get comments for {}: {}", input.issue_key, e),
                None
            )
        })?;

    let total = response.get("total").and_then(|t| t.as_u64()).unwrap_or(0);

    let comments: Vec<serde_json::Value> = response
        .get("comments")
        .and_then(|c| c.as_array())
        .map(|arr| {
            arr.iter()
                .map(|comment| {
                    let id = comment.get("id").and_then(|i| i.as_str()).unwrap_or("");
                    let author = comment
                        .get("author")
                        .and_then(|a| a.get("displayName"))
                        .and_then(|n| n.as_str())
                        .unwrap_or("Unknown");
                    let body = comment
                        .get("body")
                        .map(|b| adf_to_plain_text(b))
                        .unwrap_or_default();
                    let created = comment.get("created").and_then(|c| c.as_str()).unwrap_or("");
                    let updated = comment.get("updated").and_then(|u| u.as_str()).unwrap_or("");

                    serde_json::json!({
                        "id": id,
                        "author": author,
                        "body": body,
                        "created": created,
                        "updated": updated
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    tracing::info!(
        target: "mcp",
        tool = "get_comments",
        issue_key = %input.issue_key,
        total = total,
        returned = comments.len(),
        "Got comments successfully"
    );

    Ok(CallToolResult::structured(serde_json::json!({
        "issue_key": input.issue_key,
        "total": total,
        "comments": comments
    })))
}

pub async fn update_comment_handler(
    input: UpdateCommentInput,
    ctx: &JiraCtx,
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(
        target: "mcp",
        tool = "update_comment",
        issue_key = %input.issue_key,
        comment_id = %input.comment_id,
        "Updating comment"
    );

    ctx.client
        .update_comment(&input.issue_key, &input.comment_id, &input.body, &ctx.auth)
        .await
        .map_err(|e| {
            tracing::error!(
                target: "mcp",
                tool = "update_comment",
                error = %e,
                issue_key = %input.issue_key,
                comment_id = %input.comment_id,
                "Failed to update comment"
            );

            if let Some(jira_client::error::JiraError::ApiError { status_code, response }) = e.downcast_ref::<jira_client::error::JiraError>() {
                let error_message = extract_error_message(response);

                return rmcp::ErrorData::internal_error(
                    format!("Jira API Error ({}): {}", status_code, error_message),
                    Some(serde_json::json!({
                        "issue_key": input.issue_key,
                        "comment_id": input.comment_id,
                        "status_code": status_code,
                        "jira_response": response
                    })),
                );
            }

            rmcp::ErrorData::internal_error(
                format!("Failed to update comment {} on issue {}: {}", input.comment_id, input.issue_key, e),
                None
            )
        })?;

    tracing::info!(
        target: "mcp",
        tool = "update_comment",
        issue_key = %input.issue_key,
        comment_id = %input.comment_id,
        "Comment updated successfully"
    );

    Ok(CallToolResult::structured(
        serde_json::json!({
            "success": true,
            "issue_key": input.issue_key,
            "comment_id": input.comment_id,
            "message": format!("Comment {} on issue {} updated successfully", input.comment_id, input.issue_key)
        }),
    ))
}

pub async fn delete_comment_handler(
    input: DeleteCommentInput,
    ctx: &JiraCtx,
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(
        target: "mcp",
        tool = "delete_comment",
        issue_key = %input.issue_key,
        comment_id = %input.comment_id,
        "Deleting comment from issue"
    );

    ctx.client
        .delete_comment(&input.issue_key, &input.comment_id, &ctx.auth)
        .await
        .map_err(|e| {
            tracing::error!(
                target: "mcp",
                tool = "delete_comment",
                error = %e,
                issue_key = %input.issue_key,
                comment_id = %input.comment_id,
                "Failed to delete comment"
            );

            if let Some(jira_client::error::JiraError::ApiError { status_code, response }) = e.downcast_ref::<jira_client::error::JiraError>() {
                let error_message = extract_error_message(response);

                return rmcp::ErrorData::internal_error(
                    format!("Jira API Error ({}): {}", status_code, error_message),
                    Some(serde_json::json!({
                        "issue_key": input.issue_key,
                        "comment_id": input.comment_id,
                        "status_code": status_code,
                        "jira_response": response
                    })),
                );
            }

            rmcp::ErrorData::internal_error(
                format!("Failed to delete comment {} from issue {}: {}", input.comment_id, input.issue_key, e),
                None
            )
        })?;

    tracing::info!(
        target: "mcp",
        tool = "delete_comment",
        issue_key = %input.issue_key,
        comment_id = %input.comment_id,
        "Comment deleted successfully"
    );

    Ok(CallToolResult::structured(
        serde_json::json!({
            "success": true,
            "issue_key": input.issue_key,
            "comment_id": input.comment_id,
            "message": format!("Comment {} deleted from issue {}", input.comment_id, input.issue_key)
        }),
    ))
}
