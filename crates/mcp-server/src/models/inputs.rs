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

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddWatcherInput {
    pub issue_key: String,
    /// Account ID of the user to add as a watcher
    pub account_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RemoveWatcherInput {
    pub issue_key: String,
    /// Account ID of the user to remove as a watcher
    pub account_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct LinkIssuesInput {
    /// The issue key that is the inward (source) of the link
    pub inward_issue_key: String,
    /// The issue key that is the outward (target) of the link
    pub outward_issue_key: String,
    /// The link type name (e.g., "Blocks", "Relates", "Duplicates", "Clones")
    pub link_type: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct MoveToSprintInput {
    /// The sprint ID to move issues to
    pub sprint_id: u64,
    /// List of issue keys to move to the sprint
    pub issue_keys: Vec<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetWatchersInput {
    pub issue_key: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteIssueLinkInput {
    /// The ID of the issue link to delete
    pub link_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct MoveToBacklogInput {
    /// List of issue keys to move to the backlog
    pub issue_keys: Vec<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetSprintInput {
    /// The sprint ID to get details for
    pub sprint_id: u64,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateCommentInput {
    pub issue_key: String,
    /// The ID of the comment to update (can be obtained from get_comments)
    pub comment_id: String,
    /// The new comment body text (plain text, will be converted to ADF)
    pub body: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddLabelInput {
    pub issue_key: String,
    /// The label to add to the issue
    pub label: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RemoveLabelInput {
    pub issue_key: String,
    /// The label to remove from the issue
    pub label: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteCommentInput {
    pub issue_key: String,
    /// The ID of the comment to delete (can be obtained from get_comments)
    pub comment_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListLinkTypesInput {}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListLabelsInput {
    /// Filter labels by name (partial match). When provided, uses autocomplete suggestions endpoint.
    #[serde(default)]
    pub query: Option<String>,
    /// Index of the first item to return (0-based pagination). Only used when query is not provided.
    #[serde(default)]
    pub start_at: Option<u32>,
    /// Maximum number of labels to return (default: 1000). Only used when query is not provided.
    #[serde(default)]
    pub max_results: Option<u32>,
}

pub fn default_limit() -> usize {
    20
}

pub fn default_search_limit() -> usize {
    50
}
