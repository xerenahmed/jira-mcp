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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sprint {
    pub id: u64,
    #[serde(rename = "self")]
    pub self_url: Option<String>,
    pub state: Option<String>,
    pub name: String,
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    #[serde(rename = "completeDate")]
    pub complete_date: Option<String>,
    #[serde(rename = "originBoardId")]
    pub origin_board_id: Option<u64>,
    pub goal: Option<String>,
}
