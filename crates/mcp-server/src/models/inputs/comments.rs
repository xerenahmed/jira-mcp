use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddCommentInput {
    pub issue_key: String,
    pub body: String,
    #[serde(default)]
    pub visibility_type: Option<String>, // "role" or "group"
    #[serde(default)]
    pub visibility_value: Option<String>, // role/group name
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetCommentsInput {
    pub issue_key: String,
    #[serde(default)]
    pub max_results: Option<u32>,
    #[serde(default)]
    pub order_by: Option<String>, // e.g., "-created" for newest first
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateCommentInput {
    pub issue_key: String,
    pub comment_id: String,
    pub body: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteCommentInput {
    pub issue_key: String,
    pub comment_id: String,
}
