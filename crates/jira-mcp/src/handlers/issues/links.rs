use rmcp::model::CallToolResult;

use crate::context::JiraCtx;
use crate::errors::ResultMcpExt;
use crate::errors::suggestions::get_link_suggestions;
use crate::models::{LinkIssuesInput, DeleteIssueLinkInput};
use crate::error_ctx;
use crate::handlers::error_utils::extract_error_message;

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
        .mcp_context(
            error_ctx!("link_issues", "create issue link")
                .with_metadata("inward_issue_key", input.inward_issue_key.clone())
                .with_metadata("outward_issue_key", input.outward_issue_key.clone())
                .with_metadata("link_type", input.link_type.clone())
                .with_suggestions(get_link_suggestions)
        )?;

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
