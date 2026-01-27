use anyhow::Result;
use rmcp::model::CallToolResult;

use super::super::context::JiraCtx;
use super::super::errors::log_err;
use super::super::models::{ListProjectsInput};

pub async fn list_projects_handler(
    input: ListProjectsInput,
    ctx: &JiraCtx,
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(
        target: "mcp",
        tool = "list_projects",
        summary_only = input.summary_only,
        "Listing Jira projects"
    );

    let projects = if input.summary_only {
        ctx.client
            .list_projects_summary(&ctx.auth)
            .await
            .map_err(|e| {
                tracing::error!(
                    target: "mcp",
                    tool = "list_projects",
                    error = %e,
                    "Failed to list project summaries"
                );
                log_err("list_projects", "jira_error", e.to_string())
            })?
            .into_iter()
            .map(|p| serde_json::json!({
                "key": p.key,
                "name": p.name
            }))
            .collect::<Vec<_>>()
    } else {
        ctx.client
            .list_projects(&ctx.auth)
            .await
            .map_err(|e| {
                tracing::error!(
                    target: "mcp",
                    tool = "list_projects",
                    error = %e,
                    "Failed to list detailed projects"
                );
                log_err("list_projects", "jira_error", e.to_string())
            })?
            .into_iter()
            .map(|p| {
                let mut project_json = serde_json::json!({
                    "id": p.id,
                    "key": p.key,
                    "name": p.name,
                    "project_type_key": p.project_type_key,
                    "simplified": p.simplified,
                    "style": p.style
                });

                if let Some(description) = p.description {
                    project_json["description"] = serde_json::Value::String(description);
                }

                if let Some(lead) = p.lead {
                    project_json["lead"] = serde_json::json!({
                        "account_id": lead.account_id,
                        "display_name": lead.display_name
                    });
                }

                if let Some(url) = p.url {
                    project_json["url"] = serde_json::Value::String(url);
                }

                if let Some(category) = p.project_category {
                    project_json["project_category"] = serde_json::json!({
                        "id": category.id,
                        "name": category.name,
                        "description": category.description
                    });
                }

                project_json
            })
            .collect::<Vec<_>>()
    };

    let total_count = projects.len();

    tracing::info!(
        target: "mcp",
        tool = "list_projects",
        count = total_count,
        "Projects listed successfully"
    );

    Ok(CallToolResult::structured(
        serde_json::json!({
            "projects": projects,
            "total_count": total_count,
            "fully_paginated": true
        }),
    ))
}
