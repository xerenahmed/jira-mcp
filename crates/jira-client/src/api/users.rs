use anyhow::Result;

use crate::auth::Auth;
use crate::models::{UserInfo, UserSearchResult};
use super::ApiClient;

impl ApiClient {
    pub async fn get_myself(&self, auth: &Auth) -> Result<UserInfo> {
        let v = self.make_request(
            reqwest::Method::GET,
            "/rest/api/3/myself",
            auth,
            None,
            None,
        ).await?;

        Ok(UserInfo::from_value(v))
    }

    pub async fn search_users(
        &self,
        query: &str,
        max_results: Option<usize>,
        auth: &Auth,
    ) -> Result<Vec<UserSearchResult>> {
        tracing::debug!(target: "jira", op = "search_users", query = %query, max_results = ?max_results);

        let query_params = vec![
            ("query".into(), query.to_string()),
            ("maxResults".into(), max_results.unwrap_or(50).to_string()),
        ];

        let v = self.make_request(
            reqwest::Method::GET,
            "/rest/api/3/user/search",
            auth,
            Some(query_params),
            None,
        ).await?;

        let mut out = Vec::new();

        if let Some(users) = v.as_array() {
            for user_val in users.iter() {
                if let Some(account_id) = user_val.get("accountId").and_then(|s| s.as_str()) {
                    let account_type = user_val
                        .get("accountType")
                        .and_then(|s| s.as_str())
                        .unwrap_or("unknown")
                        .to_string();

                    let display_name = user_val
                        .get("displayName")
                        .and_then(|s| s.as_str())
                        .unwrap_or("")
                        .to_string();

                    let active = user_val
                        .get("active")
                        .and_then(|b| b.as_bool())
                        .unwrap_or(true);

                    if !display_name.is_empty() {
                        out.push(UserSearchResult {
                            account_id: account_id.to_string(),
                            account_type,
                            display_name,
                            active,
                            email_address: user_val.get("emailAddress").and_then(|s| s.as_str()).map(|s| s.to_string()),
                            time_zone: user_val.get("timeZone").and_then(|s| s.as_str()).map(|s| s.to_string()),
                            avatar_urls: user_val.get("avatarUrls").cloned(),
                        });
                    }
                }
            }
        }

        Ok(out)
    }

    pub async fn search_assignable_users(
        &self,
        query: &str,
        project_key: Option<&str>,
        issue_type: Option<&str>,
        max_results: Option<usize>,
        auth: &Auth,
    ) -> Result<Vec<UserSearchResult>> {
        tracing::debug!(target: "jira", op = "search_assignable_users", query = %query, project_key = ?project_key, issue_type = ?issue_type);

        let mut query_params = vec![
            ("query".into(), query.to_string()),
            ("maxResults".into(), max_results.unwrap_or(50).to_string()),
        ];

        if let Some(pk) = project_key {
            query_params.push(("project".into(), pk.to_string()));
        }

        if let Some(it) = issue_type {
            query_params.push(("issueType".into(), it.to_string()));
        }

        let v = self.make_request(
            reqwest::Method::GET,
            "/rest/api/3/user/assignable/search",
            auth,
            Some(query_params),
            None,
        ).await?;

        let mut out = Vec::new();

        if let Some(users) = v.as_array() {
            for user_val in users {
                if let Ok(user) = serde_json::from_value::<UserSearchResult>(user_val.clone()) {
                    out.push(user);
                }
            }
        }

        Ok(out)
    }
}
