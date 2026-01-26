use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

use crate::api::issues::CommentVisibility;
use crate::auth::Auth;
use crate::models::{Board, Issue, IssueDetail, IssueType, Project, ProjectSummary, Sprint, UserInfo, UserSearchResult};

/// Trait abstracting Jira API operations for testability
#[async_trait]
pub trait JiraApi: Send + Sync + Clone {
    // Base URL access
    fn base_url(&self) -> &url::Url;

    // Issue operations
    async fn get_createmeta(
        &self,
        project_key: Option<&str>,
        issue_type: Option<&str>,
        auth: &Auth,
    ) -> Result<Value>;

    async fn create_issue(&self, payload: &Value, auth: &Auth) -> Result<(String, String)>;

    async fn update_issue(&self, issue_key: &str, payload: &Value, auth: &Auth) -> Result<()>;

    async fn get_issue_detail(&self, key: &str, auth: &Auth) -> Result<IssueDetail>;

    async fn get_issue_editmeta(&self, key: &str, auth: &Auth) -> Result<Value>;

    async fn search_issues_fields(&self, jql: &str, limit: usize, auth: &Auth) -> Result<Vec<Issue>>;

    async fn get_recent_issues(
        &self,
        project_key: Option<&str>,
        issue_type: Option<&str>,
        limit: usize,
        epic_field_key: Option<&str>,
        auth: &Auth,
    ) -> Result<Vec<Issue>>;

    async fn search_issues(
        &self,
        jql: &str,
        fields: Option<&str>,
        limit: usize,
        start_at: Option<usize>,
        auth: &Auth,
    ) -> Result<Vec<Value>>;

    async fn list_issue_types(&self, project_key: Option<&str>, auth: &Auth) -> Result<Vec<IssueType>>;

    // Transition operations
    async fn get_transitions(&self, issue_key: &str, expand: Option<&str>, auth: &Auth) -> Result<Value>;

    async fn transition_issue(
        &self,
        issue_key: &str,
        transition_id: &str,
        fields: Option<&Value>,
        comment: Option<&str>,
        auth: &Auth,
    ) -> Result<()>;

    async fn assign_issue(&self, issue_key: &str, account_id: Option<&str>, auth: &Auth) -> Result<()>;

    // Comment operations
    async fn add_comment(
        &self,
        issue_key: &str,
        body: &str,
        visibility: Option<CommentVisibility>,
        auth: &Auth,
    ) -> Result<Value>;

    async fn get_comments(
        &self,
        issue_key: &str,
        max_results: Option<u32>,
        order_by: Option<&str>,
        auth: &Auth,
    ) -> Result<Value>;

    async fn update_comment(
        &self,
        issue_key: &str,
        comment_id: &str,
        body: &str,
        auth: &Auth,
    ) -> Result<Value>;

    async fn delete_comment(&self, issue_key: &str, comment_id: &str, auth: &Auth) -> Result<()>;

    // Watcher operations
    async fn add_watcher(&self, issue_key: &str, account_id: &str, auth: &Auth) -> Result<()>;

    async fn remove_watcher(&self, issue_key: &str, account_id: &str, auth: &Auth) -> Result<()>;

    async fn get_watchers(&self, issue_key: &str, auth: &Auth) -> Result<Value>;

    // Link operations
    async fn link_issues(
        &self,
        inward_issue_key: &str,
        outward_issue_key: &str,
        link_type: &str,
        auth: &Auth,
    ) -> Result<()>;

    async fn delete_issue_link(&self, link_id: &str, auth: &Auth) -> Result<()>;

    async fn list_link_types(&self, auth: &Auth) -> Result<Vec<Value>>;

    // Label operations
    async fn add_labels(&self, issue_key: &str, labels: &[String], auth: &Auth) -> Result<()>;

    async fn remove_labels(&self, issue_key: &str, labels: &[String], auth: &Auth) -> Result<()>;

    async fn list_labels(
        &self,
        query: Option<&str>,
        start_at: Option<u32>,
        max_results: Option<u32>,
        auth: &Auth,
    ) -> Result<Value>;

    // Board and Sprint operations
    async fn list_boards(&self, project_key: &str, auth: &Auth) -> Result<Vec<Board>>;

    async fn list_sprints(&self, board_id: u64, state: Option<&str>, auth: &Auth) -> Result<Vec<Sprint>>;

    async fn get_sprint(&self, sprint_id: u64, auth: &Auth) -> Result<Sprint>;

    async fn move_issues_to_sprint(&self, sprint_id: u64, issue_keys: &[String], auth: &Auth) -> Result<()>;

    async fn move_issues_to_backlog(&self, issue_keys: &[String], auth: &Auth) -> Result<()>;

    async fn get_board_configuration(&self, board_id: u64, auth: &Auth) -> Result<Value>;

    async fn get_filter(&self, filter_id: u64, auth: &Auth) -> Result<Value>;

    async fn get_board_issues(&self, board_id: u64, limit: usize, auth: &Auth) -> Result<Vec<Issue>>;

    // User operations
    async fn get_myself(&self, auth: &Auth) -> Result<UserInfo>;

    async fn search_users(&self, query: &str, max_results: Option<usize>, auth: &Auth) -> Result<Vec<UserSearchResult>>;

    async fn search_assignable_users(
        &self,
        query: &str,
        project_key: Option<&str>,
        issue_type: Option<&str>,
        max_results: Option<usize>,
        auth: &Auth,
    ) -> Result<Vec<UserSearchResult>>;

    // Project operations
    async fn list_projects(&self, auth: &Auth) -> Result<Vec<Project>>;

    async fn list_projects_summary(&self, auth: &Auth) -> Result<Vec<ProjectSummary>>;
}
