use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchIssuesInput {
    pub jql: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
    pub fields: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListFieldsInput {
    pub project_key: String,
    pub issue_type: String,
    #[serde(default)]
    pub field_names: Option<Vec<String>>,
    #[serde(default)]
    pub field_types: Option<Vec<String>>,
    #[serde(default)]
    pub include_required_only: bool,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetIssueInput {
    pub key: String,
    pub board_id: u64,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListIssueTypesInput {
    #[serde(default)]
    pub project_key: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetFieldDetailsInput {
    pub project_key: String,
    pub issue_type: String,
    pub field_ids: Vec<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListBoardsInput {
    pub project_key: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListProjectsInput {
    #[serde(default)]
    pub summary_only: bool,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchUsersInput {
    pub query: String,
    #[serde(default = "default_search_limit")]
    pub max_results: usize,
    #[serde(default)]
    pub project_key: Option<String>,
    #[serde(default)]
    pub issue_type: Option<String>,
    #[serde(default)]
    pub assignable_only: bool,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteCommentInput {
    pub issue_key: String,
    /// The ID of the comment to delete (can be obtained from get_comments)
    pub comment_id: String,
}

pub fn default_limit() -> usize {
    20
}

pub fn default_search_limit() -> usize {
    50
}
