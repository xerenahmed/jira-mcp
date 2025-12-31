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

    #[tool(description = "List all sprints for a board")]
    async fn list_sprints(
        &self,
        p: Parameters<ListSprintsInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let Parameters(input) = p;
        let ctx = jira_ctx()?;
        handlers::metadata::list_sprints_handler(input, &ctx).await
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

    #[tool(description = "Get available status transitions for a Jira issue")]
    async fn get_transitions(
        &self,
        p: Parameters<GetTransitionsInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let Parameters(input) = p;
        let ctx = jira_ctx()?;
        handlers::issues::get_transitions_handler(input, &ctx).await
    }

    #[tool(description = "Transition an issue to a new status")]
    async fn transition_issue(
        &self,
        p: Parameters<TransitionIssueInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let Parameters(input) = p;
        let ctx = jira_ctx()?;
        handlers::issues::transition_issue_handler(input, &ctx).await
    }

    #[tool(description = "Add a comment to a Jira issue")]
    async fn add_comment(
        &self,
        p: Parameters<AddCommentInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let Parameters(input) = p;
        let ctx = jira_ctx()?;
        handlers::issues::add_comment_handler(input, &ctx).await
    }

    #[tool(description = "Get comments from a Jira issue")]
    async fn get_comments(
        &self,
        p: Parameters<GetCommentsInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let Parameters(input) = p;
        let ctx = jira_ctx()?;
        handlers::issues::get_comments_handler(input, &ctx).await
    }

    #[tool(description = "Assign or unassign a user to/from an issue")]
    async fn assign_issue(
        &self,
        p: Parameters<AssignIssueInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let Parameters(input) = p;
        let ctx = jira_ctx()?;
        handlers::issues::assign_issue_handler(input, &ctx).await
    }

    #[tool(description = "Add a user as a watcher to an issue")]
    async fn add_watcher(
        &self,
        p: Parameters<AddWatcherInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let Parameters(input) = p;
        let ctx = jira_ctx()?;
        handlers::issues::add_watcher_handler(input, &ctx).await
    }

    #[tool(description = "Remove a user as a watcher from an issue")]
    async fn remove_watcher(
        &self,
        p: Parameters<RemoveWatcherInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let Parameters(input) = p;
        let ctx = jira_ctx()?;
        handlers::issues::remove_watcher_handler(input, &ctx).await
    }

    #[tool(description = "Create a link between two issues")]
    async fn link_issues(
        &self,
        p: Parameters<LinkIssuesInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let Parameters(input) = p;
        let ctx = jira_ctx()?;
        handlers::issues::link_issues_handler(input, &ctx).await
    }

    #[tool(description = "Move one or more issues to a sprint")]
    async fn move_to_sprint(
        &self,
        p: Parameters<MoveToSprintInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let Parameters(input) = p;
        let ctx = jira_ctx()?;
        handlers::metadata::move_to_sprint_handler(input, &ctx).await
    }

    #[tool(description = "Get all watchers for an issue")]
    async fn get_watchers(
        &self,
        p: Parameters<GetWatchersInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let Parameters(input) = p;
        let ctx = jira_ctx()?;
        handlers::issues::get_watchers_handler(input, &ctx).await
    }

    #[tool(description = "Delete a link between two issues")]
    async fn delete_issue_link(
        &self,
        p: Parameters<DeleteIssueLinkInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let Parameters(input) = p;
        let ctx = jira_ctx()?;
        handlers::issues::delete_issue_link_handler(input, &ctx).await
    }

    #[tool(description = "Move issues to the backlog (remove from sprint)")]
    async fn move_to_backlog(
        &self,
        p: Parameters<MoveToBacklogInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let Parameters(input) = p;
        let ctx = jira_ctx()?;
        handlers::metadata::move_to_backlog_handler(input, &ctx).await
    }

    #[tool(description = "Get details of a specific sprint")]
    async fn get_sprint(
        &self,
        p: Parameters<GetSprintInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let Parameters(input) = p;
        let ctx = jira_ctx()?;
        handlers::metadata::get_sprint_handler(input, &ctx).await
    }

    #[tool(description = "Update an existing comment on an issue")]
    async fn update_comment(
        &self,
        p: Parameters<UpdateCommentInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let Parameters(input) = p;
        let ctx = jira_ctx()?;
        handlers::issues::update_comment_handler(input, &ctx).await
    }

    #[tool(description = "Add a label to an issue")]
    async fn add_label(
        &self,
        p: Parameters<AddLabelInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let Parameters(input) = p;
        let ctx = jira_ctx()?;
        handlers::issues::add_label_handler(input, &ctx).await
    }

    #[tool(description = "Remove a label from an issue")]
    async fn remove_label(
        &self,
        p: Parameters<RemoveLabelInput>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let Parameters(input) = p;
        let ctx = jira_ctx()?;
        handlers::issues::remove_label_handler(input, &ctx).await
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
            instructions: Some("Jira Assistant tools: create_issue, update_issue, search_issues, list_fields, get_field_details, list_issue_types, list_boards, list_sprints, get_issue, get_user_info, list_projects, search_users, get_transitions, transition_issue, add_comment, get_comments, assign_issue, add_watcher, remove_watcher, link_issues, move_to_sprint, get_watchers, delete_issue_link, move_to_backlog, get_sprint, update_comment, add_label, remove_label, delete_comment".into()),
        }
    }
}

pub async fn serve_stdio() -> Result<()> {
    let service = JiraAssistantServer::new().serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
