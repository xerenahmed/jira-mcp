use schemars::JsonSchema;
use serde::Deserialize;

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
pub struct GetFieldDetailsInput {
    pub project_key: String,
    pub issue_type: String,
    pub field_ids: Vec<String>,
}
