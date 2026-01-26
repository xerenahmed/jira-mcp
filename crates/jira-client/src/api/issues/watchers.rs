use anyhow::Result;
use serde_json::Value;

use crate::auth::Auth;
use crate::api::ApiClient;

impl ApiClient {
    pub async fn add_watcher(
        &self,
        issue_key: &str,
        account_id: &str,
        auth: &Auth,
    ) -> Result<()> {
        tracing::info!(target: "jira", op = "add_watcher", issue_key = %issue_key, account_id = %account_id);

        let body = serde_json::Value::String(account_id.to_string());

        self.make_request(
            reqwest::Method::POST,
            &format!("/rest/api/3/issue/{}/watchers", issue_key),
            auth,
            None,
            Some(body),
        ).await?;

        Ok(())
    }

    pub async fn remove_watcher(
        &self,
        issue_key: &str,
        account_id: &str,
        auth: &Auth,
    ) -> Result<()> {
        tracing::info!(target: "jira", op = "remove_watcher", issue_key = %issue_key, account_id = %account_id);

        let query_params = vec![
            ("accountId".into(), account_id.to_string()),
        ];

        self.make_request(
            reqwest::Method::DELETE,
            &format!("/rest/api/3/issue/{}/watchers", issue_key),
            auth,
            Some(query_params),
            None,
        ).await?;

        Ok(())
    }

    pub async fn get_watchers(
        &self,
        issue_key: &str,
        auth: &Auth,
    ) -> Result<Value> {
        tracing::info!(target: "jira", op = "get_watchers", issue_key = %issue_key);

        self.make_request(
            reqwest::Method::GET,
            &format!("/rest/api/3/issue/{}/watchers", issue_key),
            auth,
            None,
            None,
        ).await
    }
}
