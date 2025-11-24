use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    pub id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default)]
    pub location: Option<BoardLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardLocation {
    #[serde(rename = "projectKey")]
    pub project_key: Option<String>,
    #[serde(rename = "projectName")]
    pub project_name: Option<String>,
}
