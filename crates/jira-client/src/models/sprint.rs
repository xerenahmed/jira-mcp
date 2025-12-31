use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sprint {
    pub id: u64,
    pub name: String,
    pub state: String,  // "future", "active", "closed"
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    #[serde(default)]
    pub goal: Option<String>,
}
