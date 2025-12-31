use std::collections::HashSet;

use anyhow::Result;
use jira_core::{CoreService, CreateIssueInput, CreateIssueResult, UpdateIssueInput, UpdateIssueResult};
use rmcp::model::CallToolResult;

use super::error_utils::{extract_error_message, get_jql_suggestions, get_create_suggestions, get_update_suggestions};
use super::super::context::JiraCtx;
use super::super::errors::log_err;
use super::super::models::{SearchIssuesInput, GetIssueInput, GetTransitionsInput, TransitionIssueInput, AddCommentInput, GetCommentsInput, AssignIssueInput, AddWatcherInput, RemoveWatcherInput, LinkIssuesInput, GetWatchersInput, DeleteIssueLinkInput, UpdateCommentInput, AddLabelInput, RemoveLabelInput, DeleteCommentInput};
use jira_client::utils::adf_collect_text;

pub async fn create_issue_handler(
    input: CreateIssueInput,
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(
        target: "mcp",
        tool = "create_issue",
        "Creating Jira issue"
    );

    let svc = CoreService::new();

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
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(
        target: "mcp",
        tool = "update_issue",
        issue_key = %input.issue_key,
        "Updating Jira issue"
    );

    let svc = CoreService::new();

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
        fields = %input.fields,
        "Searching issues"
    );

    let results = ctx
        .client
        .search_issues(&input.jql, Some(&input.fields), input.limit, &ctx.auth)
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

    let result = ctx
        .client
        .get_transitions(
            &input.issue_key,
            input.expand.as_deref(),
            &ctx.auth,
        )
        .await
        .map_err(|e| {
            tracing::error!(
                target: "mcp",
                tool = "get_transitions",
                error = %e,
                issue_key = %input.issue_key,
                "Failed to get transitions"
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
                format!("Failed to get transitions for issue {}: {}", input.issue_key, e),
                None
            )
        })?;

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

    ctx.client
        .transition_issue(
            &input.issue_key,
            &input.transition_id,
            input.fields.as_ref(),
            input.comment.as_deref(),
            &ctx.auth,
        )
        .await
        .map_err(|e| {
            tracing::error!(
                target: "mcp",
                tool = "transition_issue",
                error = %e,
                issue_key = %input.issue_key,
                transition_id = %input.transition_id,
                "Failed to transition issue"
            );

            if let Some(jira_client::error::JiraError::ApiError { status_code, response }) = e.downcast_ref::<jira_client::error::JiraError>() {
                let error_message = extract_error_message(response);

                return rmcp::ErrorData::internal_error(
                    format!("Jira API Error ({}): {}", status_code, error_message),
                    Some(serde_json::json!({
                        "issue_key": input.issue_key,
                        "transition_id": input.transition_id,
                        "status_code": status_code,
                        "jira_response": response
                    })),
                );
            }

            rmcp::ErrorData::internal_error(
                format!("Failed to transition issue {}: {}", input.issue_key, e),
                None
            )
        })?;

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

    let response = ctx
        .client
        .add_comment(&input.issue_key, &input.body, visibility, &ctx.auth)
        .await
        .map_err(|e| {
            tracing::error!(
                target: "mcp",
                tool = "add_comment",
                error = %e,
                issue_key = %input.issue_key,
                "Failed to add comment"
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
                format!("Failed to add comment to {}: {}", input.issue_key, e),
                None
            )
        })?;

    // Extract useful info from response
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

/// Convert ADF body to plain text
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

    // Extract total count
    let total = response.get("total").and_then(|t| t.as_u64()).unwrap_or(0);

    // Process comments to extract relevant fields with plain text body
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

    ctx.client
        .add_watcher(&input.issue_key, &input.account_id, &ctx.auth)
        .await
        .map_err(|e| {
            tracing::error!(
                target: "mcp",
                tool = "add_watcher",
                error = %e,
                issue_key = %input.issue_key,
                account_id = %input.account_id,
                "Failed to add watcher"
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
                format!("Failed to add watcher to issue {}: {}", input.issue_key, e),
                None
            )
        })?;

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

    ctx.client
        .remove_watcher(&input.issue_key, &input.account_id, &ctx.auth)
        .await
        .map_err(|e| {
            tracing::error!(
                target: "mcp",
                tool = "remove_watcher",
                error = %e,
                issue_key = %input.issue_key,
                account_id = %input.account_id,
                "Failed to remove watcher"
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
                format!("Failed to remove watcher from issue {}: {}", input.issue_key, e),
                None
            )
        })?;

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

pub async fn link_issues_handler(
    input: LinkIssuesInput,
    ctx: &JiraCtx,
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(
        target: "mcp",
        tool = "link_issues",
        inward_issue_key = %input.inward_issue_key,
        outward_issue_key = %input.outward_issue_key,
        link_type = %input.link_type,
        "Creating issue link"
    );

    ctx.client
        .link_issues(
            &input.inward_issue_key,
            &input.outward_issue_key,
            &input.link_type,
            &ctx.auth,
        )
        .await
        .map_err(|e| {
            tracing::error!(
                target: "mcp",
                tool = "link_issues",
                error = %e,
                inward_issue_key = %input.inward_issue_key,
                outward_issue_key = %input.outward_issue_key,
                link_type = %input.link_type,
                "Failed to create issue link"
            );

            if let Some(jira_client::error::JiraError::ApiError { status_code, response }) = e.downcast_ref::<jira_client::error::JiraError>() {
                let error_message = extract_error_message(response);

                return rmcp::ErrorData::internal_error(
                    format!("Jira API Error ({}): {}", status_code, error_message),
                    Some(serde_json::json!({
                        "inward_issue_key": input.inward_issue_key,
                        "outward_issue_key": input.outward_issue_key,
                        "link_type": input.link_type,
                        "status_code": status_code,
                        "jira_response": response
                    })),
                );
            }

            rmcp::ErrorData::internal_error(
                format!("Failed to create issue link: {}", e),
                None
            )
        })?;

    tracing::info!(
        target: "mcp",
        tool = "link_issues",
        inward_issue_key = %input.inward_issue_key,
        outward_issue_key = %input.outward_issue_key,
        link_type = %input.link_type,
        "Issue link created successfully"
    );

    Ok(CallToolResult::structured(
        serde_json::json!({
            "success": true,
            "message": format!(
                "Successfully created '{}' link: {} -> {}",
                input.link_type,
                input.inward_issue_key,
                input.outward_issue_key
            ),
            "inward_issue_key": input.inward_issue_key,
            "outward_issue_key": input.outward_issue_key,
            "link_type": input.link_type
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

pub async fn delete_issue_link_handler(
    input: DeleteIssueLinkInput,
    ctx: &JiraCtx,
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(
        target: "mcp",
        tool = "delete_issue_link",
        link_id = %input.link_id,
        "Deleting issue link"
    );

    ctx.client
        .delete_issue_link(&input.link_id, &ctx.auth)
        .await
        .map_err(|e| {
            tracing::error!(
                target: "mcp",
                tool = "delete_issue_link",
                error = %e,
                link_id = %input.link_id,
                "Failed to delete issue link"
            );

            if let Some(jira_client::error::JiraError::ApiError { status_code, response }) = e.downcast_ref::<jira_client::error::JiraError>() {
                let error_message = extract_error_message(response);

                return rmcp::ErrorData::internal_error(
                    format!("Jira API Error ({}): {}", status_code, error_message),
                    Some(serde_json::json!({
                        "link_id": input.link_id,
                        "status_code": status_code,
                        "jira_response": response
                    })),
                );
            }

            rmcp::ErrorData::internal_error(
                format!("Failed to delete issue link {}: {}", input.link_id, e),
                None
            )
        })?;

    tracing::info!(
        target: "mcp",
        tool = "delete_issue_link",
        link_id = %input.link_id,
        "Issue link deleted successfully"
    );

    Ok(CallToolResult::structured(
        serde_json::json!({
            "success": true,
            "link_id": input.link_id,
            "message": format!("Issue link {} deleted successfully", input.link_id)
        }),
    ))
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

pub async fn add_label_handler(
    input: AddLabelInput,
    ctx: &JiraCtx,
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(
        target: "mcp",
        tool = "add_label",
        issue_key = %input.issue_key,
        label = %input.label,
        "Adding label to issue"
    );

    ctx.client
        .add_label(&input.issue_key, &input.label, &ctx.auth)
        .await
        .map_err(|e| {
            tracing::error!(
                target: "mcp",
                tool = "add_label",
                error = %e,
                issue_key = %input.issue_key,
                label = %input.label,
                "Failed to add label"
            );

            if let Some(jira_client::error::JiraError::ApiError { status_code, response }) = e.downcast_ref::<jira_client::error::JiraError>() {
                let error_message = extract_error_message(response);

                return rmcp::ErrorData::internal_error(
                    format!("Jira API Error ({}): {}", status_code, error_message),
                    Some(serde_json::json!({
                        "issue_key": input.issue_key,
                        "label": input.label,
                        "status_code": status_code,
                        "jira_response": response
                    })),
                );
            }

            rmcp::ErrorData::internal_error(
                format!("Failed to add label to issue {}: {}", input.issue_key, e),
                None
            )
        })?;

    tracing::info!(
        target: "mcp",
        tool = "add_label",
        issue_key = %input.issue_key,
        label = %input.label,
        "Label added successfully"
    );

    Ok(CallToolResult::structured(
        serde_json::json!({
            "issue_key": input.issue_key,
            "label": input.label,
            "message": format!("Label '{}' added to issue {}", input.label, input.issue_key)
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
        label = %input.label,
        "Removing label from issue"
    );

    ctx.client
        .remove_label(&input.issue_key, &input.label, &ctx.auth)
        .await
        .map_err(|e| {
            tracing::error!(
                target: "mcp",
                tool = "remove_label",
                error = %e,
                issue_key = %input.issue_key,
                label = %input.label,
                "Failed to remove label"
            );

            if let Some(jira_client::error::JiraError::ApiError { status_code, response }) = e.downcast_ref::<jira_client::error::JiraError>() {
                let error_message = extract_error_message(response);

                return rmcp::ErrorData::internal_error(
                    format!("Jira API Error ({}): {}", status_code, error_message),
                    Some(serde_json::json!({
                        "issue_key": input.issue_key,
                        "label": input.label,
                        "status_code": status_code,
                        "jira_response": response
                    })),
                );
            }

            rmcp::ErrorData::internal_error(
                format!("Failed to remove label '{}' from issue {}: {}", input.label, input.issue_key, e),
                None
            )
        })?;

    tracing::info!(
        target: "mcp",
        tool = "remove_label",
        issue_key = %input.issue_key,
        label = %input.label,
        "Label removed successfully"
    );

    Ok(CallToolResult::structured(
        serde_json::json!({
            "success": true,
            "issue_key": input.issue_key,
            "removed_label": input.label
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

pub async fn list_link_types_handler(
    ctx: &JiraCtx,
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(
        target: "mcp",
        tool = "list_link_types",
        "Listing available issue link types"
    );

    let link_types = ctx
        .client
        .list_link_types(&ctx.auth)
        .await
        .map_err(|e| {
            tracing::error!(
                target: "mcp",
                tool = "list_link_types",
                error = %e,
                "Failed to list link types"
            );

            if let Some(jira_client::error::JiraError::ApiError { status_code, response }) = e.downcast_ref::<jira_client::error::JiraError>() {
                let error_message = extract_error_message(response);

                return rmcp::ErrorData::internal_error(
                    format!("Jira API Error ({}): {}", status_code, error_message),
                    Some(serde_json::json!({
                        "status_code": status_code,
                        "jira_response": response
                    })),
                );
            }

            rmcp::ErrorData::internal_error(
                format!("Failed to list link types: {}", e),
                None
            )
        })?;

    tracing::info!(
        target: "mcp",
        tool = "list_link_types",
        count = link_types.len(),
        "Link types listed successfully"
    );

    Ok(CallToolResult::structured(
        serde_json::json!({
            "link_types": link_types,
            "count": link_types.len()
        }),
    ))
}
