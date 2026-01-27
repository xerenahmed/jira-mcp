use schemars::JsonSchema;
use serde::Deserialize;

use super::default_search_limit;

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
