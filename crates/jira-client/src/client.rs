use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use url::Url;

use crate::api::ApiClient;
use crate::api::issues::CommentVisibility;
use crate::auth::Auth;
use crate::config::JiraConfig;
use crate::models::*;
use crate::traits::JiraApi;

#[derive(Clone)]
pub struct JiraClient {
    api_client: ApiClient,
}

impl JiraClient {
    pub fn new(base_url: impl AsRef<str>, _auth: Auth) -> Result<Self> {
        let url = Url::parse(base_url.as_ref())?;

        let client = reqwest::Client::builder()
            .user_agent("jira-mcp/0.1")
            .build()?;

        let api_client = ApiClient::new(url, client);

        Ok(Self { api_client })
    }

    pub fn from_config(cfg: &JiraConfig) -> Result<Self> {
        Self::new(&cfg.jira_base_url, cfg.create_auth())
    }

    pub fn base_url(&self) -> &url::Url {
        &self.api_client.base_url
    }

    pub async fn get_createmeta(
        &self,
        project_key: Option<&str>,
        issue_type: Option<&str>,
        auth: &Auth,
    ) -> Result<serde_json::Value> {
        self.api_client.get_createmeta(project_key, issue_type, auth).await
    }

    pub async fn create_issue(
        &self,
        payload: &serde_json::Value,
        auth: &Auth,
    ) -> Result<(String, String)> {
        self.api_client.create_issue(payload, auth).await
    }

    pub async fn update_issue(
        &self,
        issue_key: &str,
        payload: &serde_json::Value,
        auth: &Auth,
    ) -> Result<()> {
        self.api_client.update_issue(issue_key, payload, auth).await
    }

    pub async fn get_issue_detail(&self, key: &str, auth: &Auth) -> Result<IssueDetail> {
        self.api_client.get_issue_detail(key, auth).await
    }

    pub async fn get_issue_editmeta(&self, key: &str, auth: &Auth) -> Result<serde_json::Value> {
        self.api_client.get_issue_editmeta(key, auth).await
    }

    pub async fn search_issues_fields(
        &self,
        jql: &str,
        limit: usize,
        auth: &Auth,
    ) -> Result<Vec<Issue>> {
        self.api_client.search_issues_fields(jql, limit, auth).await
    }

    pub async fn get_recent_issues(
        &self,
        project_key: Option<&str>,
        issue_type: Option<&str>,
        limit: usize,
        epic_field_key: Option<&str>,
        auth: &Auth,
    ) -> Result<Vec<Issue>> {
        self.api_client.get_recent_issues(project_key, issue_type, limit, epic_field_key, auth).await
    }

    pub async fn search_issues(
        &self,
        jql: &str,
        fields: Option<&str>,
        limit: usize,
        start_at: Option<usize>,
        auth: &Auth,
    ) -> Result<Vec<serde_json::Value>> {
        self.api_client.search_issues(jql, fields, limit, start_at, auth).await
    }

    pub async fn list_issue_types(
        &self,
        project_key: Option<&str>,
        auth: &Auth,
    ) -> Result<Vec<IssueType>> {
        self.api_client.list_issue_types(project_key, auth).await
    }

    pub async fn get_transitions(
        &self,
        issue_key: &str,
        expand: Option<&str>,
        auth: &Auth,
    ) -> Result<serde_json::Value> {
        self.api_client.get_transitions(issue_key, expand, auth).await
    }

    pub async fn transition_issue(
        &self,
        issue_key: &str,
        transition_id: &str,
        fields: Option<&serde_json::Value>,
        comment: Option<&str>,
        auth: &Auth,
    ) -> Result<()> {
        self.api_client.transition_issue(issue_key, transition_id, fields, comment, auth).await
    }

    pub async fn get_comments(
        &self,
        issue_key: &str,
        max_results: Option<u32>,
        order_by: Option<&str>,
        auth: &Auth,
    ) -> Result<serde_json::Value> {
        self.api_client.get_comments(issue_key, max_results, order_by, auth).await
    }

    pub async fn assign_issue(
        &self,
        issue_key: &str,
        account_id: Option<&str>,
        auth: &Auth,
    ) -> Result<()> {
        self.api_client.assign_issue(issue_key, account_id, auth).await
    }

    pub async fn add_watcher(
        &self,
        issue_key: &str,
        account_id: &str,
        auth: &Auth,
    ) -> Result<()> {
        self.api_client.add_watcher(issue_key, account_id, auth).await
    }

    pub async fn remove_watcher(
        &self,
        issue_key: &str,
        account_id: &str,
        auth: &Auth,
    ) -> Result<()> {
        self.api_client.remove_watcher(issue_key, account_id, auth).await
    }

    pub async fn link_issues(
        &self,
        inward_issue_key: &str,
        outward_issue_key: &str,
        link_type: &str,
        auth: &Auth,
    ) -> Result<()> {
        self.api_client.link_issues(inward_issue_key, outward_issue_key, link_type, auth).await
    }

    pub async fn get_board_configuration(
        &self,
        board_id: u64,
        auth: &Auth,
    ) -> Result<serde_json::Value> {
        self.api_client.get_board_configuration(board_id, auth).await
    }

    pub async fn get_filter(&self, filter_id: u64, auth: &Auth) -> Result<serde_json::Value> {
        self.api_client.get_filter(filter_id, auth).await
    }

    pub async fn get_board_issues(
        &self,
        board_id: u64,
        limit: usize,
        auth: &Auth,
    ) -> Result<Vec<Issue>> {
        self.api_client.get_board_issues(board_id, limit, auth).await
    }

    pub async fn list_boards(
        &self,
        project_key: &str,
        auth: &Auth,
    ) -> Result<Vec<Board>> {
        self.api_client.list_boards(project_key, auth).await
    }

    pub async fn list_sprints(
        &self,
        board_id: u64,
        state: Option<&str>,
        auth: &Auth,
    ) -> Result<Vec<Sprint>> {
        self.api_client.list_sprints(board_id, state, auth).await
    }

    pub async fn get_myself(&self, auth: &Auth) -> Result<UserInfo> {
        self.api_client.get_myself(auth).await
    }

    pub async fn search_users(
        &self,
        query: &str,
        max_results: Option<usize>,
        auth: &Auth,
    ) -> Result<Vec<UserSearchResult>> {
        self.api_client.search_users(query, max_results, auth).await
    }

    pub async fn search_assignable_users(
        &self,
        query: &str,
        project_key: Option<&str>,
        issue_type: Option<&str>,
        max_results: Option<usize>,
        auth: &Auth,
    ) -> Result<Vec<UserSearchResult>> {
        self.api_client.search_assignable_users(query, project_key, issue_type, max_results, auth).await
    }

    pub async fn list_projects(&self, auth: &Auth) -> Result<Vec<Project>> {
        self.api_client.list_projects(auth).await
    }

    pub async fn list_projects_summary(&self, auth: &Auth) -> Result<Vec<ProjectSummary>> {
        self.api_client.list_projects_summary(auth).await
    }

    pub async fn add_comment(
        &self,
        issue_key: &str,
        body: &str,
        visibility: Option<crate::api::issues::CommentVisibility>,
        auth: &Auth,
    ) -> Result<serde_json::Value> {
        self.api_client.add_comment(issue_key, body, visibility, auth).await
    }

    pub async fn move_issues_to_sprint(
        &self,
        sprint_id: u64,
        issue_keys: &[String],
        auth: &Auth,
    ) -> Result<()> {
        self.api_client.move_issues_to_sprint(sprint_id, issue_keys, auth).await
    }

    pub async fn get_watchers(
        &self,
        issue_key: &str,
        auth: &Auth,
    ) -> Result<serde_json::Value> {
        self.api_client.get_watchers(issue_key, auth).await
    }

    pub async fn delete_issue_link(
        &self,
        link_id: &str,
        auth: &Auth,
    ) -> Result<()> {
        self.api_client.delete_issue_link(link_id, auth).await
    }

    pub async fn move_issues_to_backlog(
        &self,
        issue_keys: &[String],
        auth: &Auth,
    ) -> Result<()> {
        self.api_client.move_issues_to_backlog(issue_keys, auth).await
    }

    pub async fn get_sprint(
        &self,
        sprint_id: u64,
        auth: &Auth,
    ) -> Result<Sprint> {
        self.api_client.get_sprint(sprint_id, auth).await
    }

    pub async fn update_comment(
        &self,
        issue_key: &str,
        comment_id: &str,
        body: &str,
        auth: &Auth,
    ) -> Result<serde_json::Value> {
        self.api_client.update_comment(issue_key, comment_id, body, auth).await
    }

    pub async fn add_labels(
        &self,
        issue_key: &str,
        labels: &[String],
        auth: &Auth,
    ) -> Result<()> {
        self.api_client.add_labels(issue_key, labels, auth).await
    }

    pub async fn remove_labels(
        &self,
        issue_key: &str,
        labels: &[String],
        auth: &Auth,
    ) -> Result<()> {
        self.api_client.remove_labels(issue_key, labels, auth).await
    }

    pub async fn delete_comment(
        &self,
        issue_key: &str,
        comment_id: &str,
        auth: &Auth,
    ) -> Result<()> {
        self.api_client.delete_comment(issue_key, comment_id, auth).await
    }

    pub async fn list_link_types(&self, auth: &Auth) -> Result<Vec<serde_json::Value>> {
        self.api_client.list_link_types(auth).await
    }

    pub async fn list_labels(
        &self,
        query: Option<&str>,
        start_at: Option<u32>,
        max_results: Option<u32>,
        auth: &Auth,
    ) -> Result<serde_json::Value> {
        self.api_client.list_labels(query, start_at, max_results, auth).await
    }
}

#[async_trait]
impl JiraApi for JiraClient {
    fn base_url(&self) -> &url::Url {
        &self.api_client.base_url
    }

    async fn get_createmeta(
        &self,
        project_key: Option<&str>,
        issue_type: Option<&str>,
        auth: &Auth,
    ) -> Result<Value> {
        self.api_client.get_createmeta(project_key, issue_type, auth).await
    }

    async fn create_issue(&self, payload: &Value, auth: &Auth) -> Result<(String, String)> {
        self.api_client.create_issue(payload, auth).await
    }

    async fn update_issue(&self, issue_key: &str, payload: &Value, auth: &Auth) -> Result<()> {
        self.api_client.update_issue(issue_key, payload, auth).await
    }

    async fn get_issue_detail(&self, key: &str, auth: &Auth) -> Result<IssueDetail> {
        self.api_client.get_issue_detail(key, auth).await
    }

    async fn get_issue_editmeta(&self, key: &str, auth: &Auth) -> Result<Value> {
        self.api_client.get_issue_editmeta(key, auth).await
    }

    async fn search_issues_fields(&self, jql: &str, limit: usize, auth: &Auth) -> Result<Vec<Issue>> {
        self.api_client.search_issues_fields(jql, limit, auth).await
    }

    async fn get_recent_issues(
        &self,
        project_key: Option<&str>,
        issue_type: Option<&str>,
        limit: usize,
        epic_field_key: Option<&str>,
        auth: &Auth,
    ) -> Result<Vec<Issue>> {
        self.api_client.get_recent_issues(project_key, issue_type, limit, epic_field_key, auth).await
    }

    async fn search_issues(
        &self,
        jql: &str,
        fields: Option<&str>,
        limit: usize,
        start_at: Option<usize>,
        auth: &Auth,
    ) -> Result<Vec<Value>> {
        self.api_client.search_issues(jql, fields, limit, start_at, auth).await
    }

    async fn list_issue_types(&self, project_key: Option<&str>, auth: &Auth) -> Result<Vec<IssueType>> {
        self.api_client.list_issue_types(project_key, auth).await
    }

    async fn get_transitions(&self, issue_key: &str, expand: Option<&str>, auth: &Auth) -> Result<Value> {
        self.api_client.get_transitions(issue_key, expand, auth).await
    }

    async fn transition_issue(
        &self,
        issue_key: &str,
        transition_id: &str,
        fields: Option<&Value>,
        comment: Option<&str>,
        auth: &Auth,
    ) -> Result<()> {
        self.api_client.transition_issue(issue_key, transition_id, fields, comment, auth).await
    }

    async fn assign_issue(&self, issue_key: &str, account_id: Option<&str>, auth: &Auth) -> Result<()> {
        self.api_client.assign_issue(issue_key, account_id, auth).await
    }

    async fn add_comment(
        &self,
        issue_key: &str,
        body: &str,
        visibility: Option<CommentVisibility>,
        auth: &Auth,
    ) -> Result<Value> {
        self.api_client.add_comment(issue_key, body, visibility, auth).await
    }

    async fn get_comments(
        &self,
        issue_key: &str,
        max_results: Option<u32>,
        order_by: Option<&str>,
        auth: &Auth,
    ) -> Result<Value> {
        self.api_client.get_comments(issue_key, max_results, order_by, auth).await
    }

    async fn update_comment(
        &self,
        issue_key: &str,
        comment_id: &str,
        body: &str,
        auth: &Auth,
    ) -> Result<Value> {
        self.api_client.update_comment(issue_key, comment_id, body, auth).await
    }

    async fn delete_comment(&self, issue_key: &str, comment_id: &str, auth: &Auth) -> Result<()> {
        self.api_client.delete_comment(issue_key, comment_id, auth).await
    }

    async fn add_watcher(&self, issue_key: &str, account_id: &str, auth: &Auth) -> Result<()> {
        self.api_client.add_watcher(issue_key, account_id, auth).await
    }

    async fn remove_watcher(&self, issue_key: &str, account_id: &str, auth: &Auth) -> Result<()> {
        self.api_client.remove_watcher(issue_key, account_id, auth).await
    }

    async fn get_watchers(&self, issue_key: &str, auth: &Auth) -> Result<Value> {
        self.api_client.get_watchers(issue_key, auth).await
    }

    async fn link_issues(
        &self,
        inward_issue_key: &str,
        outward_issue_key: &str,
        link_type: &str,
        auth: &Auth,
    ) -> Result<()> {
        self.api_client.link_issues(inward_issue_key, outward_issue_key, link_type, auth).await
    }

    async fn delete_issue_link(&self, link_id: &str, auth: &Auth) -> Result<()> {
        self.api_client.delete_issue_link(link_id, auth).await
    }

    async fn list_link_types(&self, auth: &Auth) -> Result<Vec<Value>> {
        self.api_client.list_link_types(auth).await
    }

    async fn add_labels(&self, issue_key: &str, labels: &[String], auth: &Auth) -> Result<()> {
        self.api_client.add_labels(issue_key, labels, auth).await
    }

    async fn remove_labels(&self, issue_key: &str, labels: &[String], auth: &Auth) -> Result<()> {
        self.api_client.remove_labels(issue_key, labels, auth).await
    }

    async fn list_labels(
        &self,
        query: Option<&str>,
        start_at: Option<u32>,
        max_results: Option<u32>,
        auth: &Auth,
    ) -> Result<Value> {
        self.api_client.list_labels(query, start_at, max_results, auth).await
    }

    async fn list_boards(&self, project_key: &str, auth: &Auth) -> Result<Vec<Board>> {
        self.api_client.list_boards(project_key, auth).await
    }

    async fn list_sprints(&self, board_id: u64, state: Option<&str>, auth: &Auth) -> Result<Vec<Sprint>> {
        self.api_client.list_sprints(board_id, state, auth).await
    }

    async fn get_sprint(&self, sprint_id: u64, auth: &Auth) -> Result<Sprint> {
        self.api_client.get_sprint(sprint_id, auth).await
    }

    async fn move_issues_to_sprint(&self, sprint_id: u64, issue_keys: &[String], auth: &Auth) -> Result<()> {
        self.api_client.move_issues_to_sprint(sprint_id, issue_keys, auth).await
    }

    async fn move_issues_to_backlog(&self, issue_keys: &[String], auth: &Auth) -> Result<()> {
        self.api_client.move_issues_to_backlog(issue_keys, auth).await
    }

    async fn get_board_configuration(&self, board_id: u64, auth: &Auth) -> Result<Value> {
        self.api_client.get_board_configuration(board_id, auth).await
    }

    async fn get_filter(&self, filter_id: u64, auth: &Auth) -> Result<Value> {
        self.api_client.get_filter(filter_id, auth).await
    }

    async fn get_board_issues(&self, board_id: u64, limit: usize, auth: &Auth) -> Result<Vec<Issue>> {
        self.api_client.get_board_issues(board_id, limit, auth).await
    }

    async fn get_myself(&self, auth: &Auth) -> Result<UserInfo> {
        self.api_client.get_myself(auth).await
    }

    async fn search_users(&self, query: &str, max_results: Option<usize>, auth: &Auth) -> Result<Vec<UserSearchResult>> {
        self.api_client.search_users(query, max_results, auth).await
    }

    async fn search_assignable_users(
        &self,
        query: &str,
        project_key: Option<&str>,
        issue_type: Option<&str>,
        max_results: Option<usize>,
        auth: &Auth,
    ) -> Result<Vec<UserSearchResult>> {
        self.api_client.search_assignable_users(query, project_key, issue_type, max_results, auth).await
    }

    async fn list_projects(&self, auth: &Auth) -> Result<Vec<Project>> {
        self.api_client.list_projects(auth).await
    }

    async fn list_projects_summary(&self, auth: &Auth) -> Result<Vec<ProjectSummary>> {
        self.api_client.list_projects_summary(auth).await
    }
}
