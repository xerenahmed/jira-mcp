use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSummary {
    pub key: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub key: String,
    pub name: String,
    pub project_type_key: String,
    pub simplified: bool,
    pub style: String,
    #[serde(default)]
    pub avatar_urls: Option<serde_json::Value>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub lead: Option<UserRef>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub project_category: Option<ProjectCategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRef {
    pub account_id: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectCategory {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}
