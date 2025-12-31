use anyhow::Result;
use url::Url;

use crate::api::ApiClient;
use crate::auth::Auth;
use crate::config::JiraConfig;
use crate::models::*;

#[derive(Clone)]
pub struct JiraClient {
    api_client: ApiClient,
}

impl JiraClient {
    pub fn new(base_url: impl AsRef<str>, auth: Auth) -> Result<Self> {
        let url = Url::parse(base_url.as_ref())?;

        let client = match auth {
            Auth::Basic {
                username: _,
                token: _,
            } => {
                let mut builder = reqwest::Client::builder();
                builder = builder.user_agent("jira-mcp/0.1");
                builder.build()?
            }
            Auth::Bearer { token: _ } => {
                reqwest::Client::builder()
                    .user_agent("jira-mcp/0.1")
                    .build()?
            }
            Auth::None => reqwest::Client::new(),
        };

        let api_client = ApiClient::new(url, client);

        Ok(Self { api_client })
    }

    pub fn from_config(cfg: JiraConfig) -> Result<Self> {
        let auth = match (
            &cfg.auth.method[..],
            cfg.auth.username.clone(),
            cfg.auth.token.clone(),
        ) {
            ("pat", Some(user), Some(token)) => Auth::Basic {
                username: user,
                token,
            },
            ("bearer", _, Some(token)) => Auth::Bearer { token },
            _ => Auth::None,
        };

        Self::new(&cfg.jira_base_url, auth)
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
        auth: &Auth,
    ) -> Result<Vec<serde_json::Value>> {
        self.api_client.search_issues(jql, fields, limit, auth).await
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
}
