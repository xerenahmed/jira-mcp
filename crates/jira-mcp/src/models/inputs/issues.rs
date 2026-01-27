use schemars::JsonSchema;
use serde::Deserialize;

use super::default_limit;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchIssuesInput {
    pub jql: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
    pub fields: String,
    #[serde(default)]
    pub start_at: Option<usize>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetIssueInput {
    pub key: String,
    pub board_id: u64,
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
    pub transition_id: String,
    #[serde(default)]
    pub fields: Option<serde_json::Value>,
    #[serde(default)]
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AssignIssueInput {
    pub issue_key: String,
    #[serde(default)]
    pub account_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddWatcherInput {
    pub issue_key: String,
    pub account_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RemoveWatcherInput {
    pub issue_key: String,
    pub account_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetWatchersInput {
    pub issue_key: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct LinkIssuesInput {
    pub inward_issue_key: String,
    pub outward_issue_key: String,
    pub link_type: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteIssueLinkInput {
    pub link_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListLinkTypesInput {}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddLabelInput {
    pub issue_key: String,
    pub labels: Vec<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RemoveLabelInput {
    pub issue_key: String,
    pub labels: Vec<String>,
}
