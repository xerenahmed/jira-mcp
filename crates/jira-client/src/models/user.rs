use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub account_id: Option<String>,
    pub account_type: Option<String>,
    pub email_address: Option<String>,
    pub display_name: Option<String>,
    pub time_zone: Option<String>,
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSearchResult {
    pub account_id: String,
    pub account_type: String,
    pub display_name: String,
    pub active: bool,
    #[serde(default)]
    pub email_address: Option<String>,
    #[serde(default)]
    pub time_zone: Option<String>,
    #[serde(default)]
    pub avatar_urls: Option<serde_json::Value>,
}

impl UserInfo {
    pub fn from_value(v: serde_json::Value) -> Self {
        let account_id = v
            .get("accountId")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string());

        let account_type = v
            .get("accountType")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string());

        let email_address = v
            .get("emailAddress")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string());

        let display_name = v
            .get("displayName")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string());

        let time_zone = v
            .get("timeZone")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string());

        let active = v.get("active").and_then(|b| b.as_bool());

        UserInfo {
            account_id,
            account_type,
            email_address,
            display_name,
            time_zone,
            active,
        }
    }
}
