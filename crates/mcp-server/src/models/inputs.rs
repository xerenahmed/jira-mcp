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
pub struct GetTransitionsInput {
    pub issue_key: String,
    #[serde(default)]
    pub expand: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TransitionIssueInput {
    pub issue_key: String,
    /// The transition ID (from get_transitions)
    pub transition_id: String,
    /// Optional fields to set during transition
    #[serde(default)]
    pub fields: Option<serde_json::Value>,
    /// Optional comment to add (plain text, converted to ADF)
    #[serde(default)]
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddCommentInput {
    pub issue_key: String,
    pub body: String,
    #[serde(default)]
    pub visibility_type: Option<String>,  // "role" or "group"
    #[serde(default)]
    pub visibility_value: Option<String>, // role/group name
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetCommentsInput {
    pub issue_key: String,
    #[serde(default)]
    pub max_results: Option<u32>,
    #[serde(default)]
    pub order_by: Option<String>,  // e.g., "-created" for newest first
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListSprintsInput {
    pub board_id: u64,
    #[serde(default)]
    pub state: Option<String>,  // "future", "active", "closed"
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AssignIssueInput {
    pub issue_key: String,
    /// Account ID of the user to assign. Set to null or omit to unassign.
    #[serde(default)]
    pub account_id: Option<String>,
}

pub fn default_limit() -> usize {
    20
}

pub fn default_search_limit() -> usize {
    50
}
