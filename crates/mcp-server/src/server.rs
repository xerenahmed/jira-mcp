use anyhow::Result;
use jira_core::CreateIssueInput;
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{CallToolResult, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
    transport::stdio,
    ServerHandler, ServiceExt,
};
use super::models::*;
use super::context::jira_ctx;
use super::{handlers};
use jira_core::UpdateIssueInput;

#[derive(Clone)]
pub struct JiraAssistantServer {
    tool_router: ToolRouter<Self>,
}

impl Default for JiraAssistantServer {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_router]
impl JiraAssistantServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Create a Jira issue")]
    async fn create_issue(
        &self,
        p: Parameters<CreateIssueInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let Parameters(input) = p;
        handlers::issues::create_issue_handler(input).await
    }

    #[tool(description = "Update a Jira issue with specified fields")]
    async fn update_issue(
        &self,
        p: Parameters<UpdateIssueInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let Parameters(input) = p;
        handlers::issues::update_issue_handler(input).await
    }

    #[tool(description = "Search issues by JQL query")]
    async fn search_issues(
        &self,
        p: Parameters<SearchIssuesInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let Parameters(input) = p;
        let ctx = jira_ctx()?;
        handlers::issues::search_issues_handler(input, &ctx).await
    }

    #[tool(description = "List fields for a project and issue type (id, name, type, required/optional)")]
    async fn list_fields(
        &self,
        p: Parameters<ListFieldsInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let Parameters(input) = p;
        let ctx = jira_ctx()?;
        handlers::fields::list_fields_handler(input, &ctx).await
    }

    #[tool(description = "Get detailed field information (schema, allowed_values) for specific fields")]
    async fn get_field_details(
        &self,
        p: Parameters<GetFieldDetailsInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let Parameters(input) = p;
        let ctx = jira_ctx()?;
        handlers::fields::get_field_details_handler(input, &ctx).await
    }

    #[tool(description = "Get a Jira issue with full fields (including custom), plus name mapping and schema")]
    async fn get_issue(
        &self,
        p: Parameters<GetIssueInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let Parameters(input) = p;
        let ctx = jira_ctx()?;
        handlers::issues::get_issue_handler(input, &ctx).await
    }

    #[tool(description = "Get current authenticated user info (account id, display name, etc.)")]
    async fn get_user_info(&self) -> Result<CallToolResult, rmcp::ErrorData> {
        let ctx = jira_ctx()?;
        handlers::metadata::get_user_info_handler(&ctx).await
    }

    #[tool(description = "List issue types globally or for a project")]
    async fn list_issue_types(
        &self,
        p: Parameters<ListIssueTypesInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let Parameters(input) = p;
        let ctx = jira_ctx()?;
        handlers::metadata::list_issue_types_handler(input, &ctx).await
    }

    #[tool(description = "List all boards for a specific project")]
    async fn list_boards(
        &self,
        p: Parameters<ListBoardsInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let Parameters(p) = p;
        let ctx = jira_ctx()?;
        handlers::metadata::list_boards_handler(p.project_key, &ctx).await
    }

    #[tool(description = "List all projects the user has access to")]
    async fn list_projects(
        &self,
        p: Parameters<ListProjectsInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let Parameters(input) = p;
        let ctx = jira_ctx()?;
        handlers::projects::list_projects_handler(input, &ctx).await
    }

    #[tool(description = "Search for users by name, email, or display name")]
    async fn search_users(
        &self,
        p: Parameters<SearchUsersInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let Parameters(input) = p;
        let ctx = jira_ctx()?;
        handlers::users::search_users_handler(input, &ctx).await
    }

    #[tool(description = "Delete a comment from an issue")]
    async fn delete_comment(
        &self,
        p: Parameters<DeleteCommentInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let Parameters(input) = p;
        let ctx = jira_ctx()?;
        handlers::issues::delete_comment_handler(input, &ctx).await
    }
}

#[tool_handler]
impl ServerHandler for JiraAssistantServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2025_06_18,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("Jira Assistant tools: create_issue, update_issue, search_issues, list_fields, get_field_details, list_issue_types, list_boards, get_issue, get_user_info, list_projects, search_users, delete_comment".into()),
        }
    }
}

pub async fn serve_stdio() -> Result<()> {
    let service = JiraAssistantServer::new().serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
