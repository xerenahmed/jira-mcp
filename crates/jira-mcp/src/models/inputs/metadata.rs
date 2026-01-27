use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListIssueTypesInput {
    #[serde(default)]
    pub project_key: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListBoardsInput {
    pub project_key: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListSprintsInput {
    pub board_id: u64,
    #[serde(default)]
    pub state: Option<String>, // "future", "active", "closed"
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetSprintInput {
    pub sprint_id: u64,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct MoveToSprintInput {
    pub sprint_id: u64,
    pub issue_keys: Vec<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct MoveToBacklogInput {
    pub issue_keys: Vec<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListLabelsInput {
    #[serde(default)]
    pub query: Option<String>,
    #[serde(default)]
    pub start_at: Option<u32>,
    #[serde(default)]
    pub max_results: Option<u32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListProjectsInput {
    #[serde(default)]
    pub summary_only: bool,
}
