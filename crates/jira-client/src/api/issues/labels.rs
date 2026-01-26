use anyhow::Result;
use serde_json::{json, Value};

use crate::auth::Auth;
use crate::api::ApiClient;

impl ApiClient {
    pub async fn add_labels(
        &self,
        issue_key: &str,
        labels: &[String],
        auth: &Auth,
    ) -> Result<()> {
        tracing::info!(target: "jira", op = "add_labels", issue_key = %issue_key, labels = ?labels);

        let label_ops: Vec<_> = labels.iter().map(|l| json!({ "add": l })).collect();

        let body = json!({
            "update": {
                "labels": label_ops
            }
        });

        self.make_request(
            reqwest::Method::PUT,
            &format!("/rest/api/3/issue/{}", issue_key),
            auth,
            None,
            Some(body),
        ).await?;

        Ok(())
    }

    pub async fn remove_labels(
        &self,
        issue_key: &str,
        labels: &[String],
        auth: &Auth,
    ) -> Result<()> {
        tracing::info!(target: "jira", op = "remove_labels", issue_key = %issue_key, labels = ?labels);

        let label_ops: Vec<_> = labels.iter().map(|l| json!({ "remove": l })).collect();

        let body = json!({
            "update": {
                "labels": label_ops
            }
        });

        self.make_request(
            reqwest::Method::PUT,
            &format!("/rest/api/3/issue/{}", issue_key),
            auth,
            None,
            Some(body),
        ).await?;

        Ok(())
    }

    pub async fn list_labels(
        &self,
        query: Option<&str>,
        start_at: Option<u32>,
        max_results: Option<u32>,
        auth: &Auth,
    ) -> Result<Value> {
        tracing::info!(target: "jira", op = "list_labels", query = ?query, start_at = ?start_at, max_results = ?max_results);

        if let Some(q) = query {
            let mut query_params: Vec<(String, String)> = vec![
                ("fieldName".into(), "labels".into()),
            ];

            if !q.is_empty() {
                query_params.push(("fieldValue".into(), q.to_string()));
            }

            let response: Value = self.make_request(
                reqwest::Method::GET,
                "/rest/api/3/jql/autocompletedata/suggestions",
                auth,
                Some(query_params),
                None,
            ).await?;

            let labels: Vec<String> = response
                .get("results")
                .and_then(|r| r.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|item| item.get("value").and_then(|v| v.as_str()))
                        .map(|s| s.to_string())
                        .collect()
                })
                .unwrap_or_default();

            return Ok(json!({
                "labels": labels,
                "count": labels.len(),
                "filtered": true,
                "query": q
            }));
        }

        let mut query_params: Vec<(String, String)> = Vec::new();

        if let Some(start) = start_at {
            query_params.push(("startAt".into(), start.to_string()));
        }

        if let Some(max) = max_results {
            query_params.push(("maxResults".into(), max.to_string()));
        }

        let params = if query_params.is_empty() {
            None
        } else {
            Some(query_params)
        };

        self.make_request(
            reqwest::Method::GET,
            "/rest/api/3/label",
            auth,
            params,
            None,
        ).await
    }
}
