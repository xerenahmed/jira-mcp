use schemars::{JsonSchema, Schema, SchemaGenerator};
use serde::{Deserialize, Serialize};

fn json_object_schema(_gen: &mut SchemaGenerator) -> Schema {
    serde_json::from_value(serde_json::json!({
        "type": "object",
        "description": "A JSON object with Jira field key-value pairs",
        "additionalProperties": true
    }))
    .expect("valid schema")
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateIssueInput {
    #[schemars(schema_with = "json_object_schema")]
    pub fields: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreatedIssue {
    pub issue_key: String,
    pub url: String,
    pub actions: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "result", rename_all = "snake_case")]
pub enum CreateIssueResult {
    Created(CreatedIssue),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UpdateIssueInput {
    pub issue_key: String,
    #[schemars(schema_with = "json_object_schema")]
    pub fields: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UpdatedIssue {
    pub issue_key: String,
    pub url: String,
    pub updated_fields: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "result", rename_all = "snake_case")]
pub enum UpdateIssueResult {
    Updated(UpdatedIssue),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FieldDef {
    pub id: String,
    pub name: String,
    pub required: bool,
    pub schema: serde_json::Value,
    #[serde(default, rename = "allowed_values")]
    pub allowed_values: serde_json::Value,
}
