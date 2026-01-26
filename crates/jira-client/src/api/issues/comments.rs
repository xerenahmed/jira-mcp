use anyhow::Result;
use serde_json::{json, Value};

use crate::auth::Auth;
use crate::api::ApiClient;

use super::utils::{text_to_adf, CommentVisibility};

impl ApiClient {
    pub async fn add_comment(
        &self,
        issue_key: &str,
        body: &str,
        visibility: Option<CommentVisibility>,
        auth: &Auth,
    ) -> Result<Value> {
        tracing::info!(target: "jira", op = "add_comment", issue_key = %issue_key);

        let adf_body = text_to_adf(body);

        let mut payload = json!({
            "body": adf_body
        });

        if let Some(vis) = visibility {
            payload.as_object_mut().unwrap().insert(
                "visibility".to_string(),
                json!({
                    "type": vis.visibility_type,
                    "value": vis.value
                }),
            );
        }

        self.make_request(
            reqwest::Method::POST,
            &format!("/rest/api/3/issue/{}/comment", issue_key),
            auth,
            None,
            Some(payload),
        ).await
    }

    pub async fn get_comments(
        &self,
        issue_key: &str,
        max_results: Option<u32>,
        order_by: Option<&str>,
        auth: &Auth,
    ) -> Result<Value> {
        tracing::info!(target: "jira", op = "get_comments", issue_key = %issue_key, max_results = ?max_results, order_by = ?order_by);

        let mut query_params: Vec<(String, String)> = Vec::new();

        if let Some(max) = max_results {
            query_params.push(("maxResults".into(), max.to_string()));
        }

        let order = order_by.unwrap_or("-created");
        query_params.push(("orderBy".into(), order.to_string()));

        let query = if query_params.is_empty() {
            None
        } else {
            Some(query_params)
        };

        self.make_request(
            reqwest::Method::GET,
            &format!("/rest/api/3/issue/{}/comment", issue_key),
            auth,
            query,
            None,
        ).await
    }

    pub async fn update_comment(
        &self,
        issue_key: &str,
        comment_id: &str,
        body: &str,
        auth: &Auth,
    ) -> Result<Value> {
        tracing::info!(target: "jira", op = "update_comment", issue_key = %issue_key, comment_id = %comment_id);

        let adf_body = serde_json::json!({
            "body": {
                "type": "doc",
                "version": 1,
                "content": [
                    {
                        "type": "paragraph",
                        "content": [
                            {
                                "type": "text",
                                "text": body
                            }
                        ]
                    }
                ]
            }
        });

        self.make_request(
            reqwest::Method::PUT,
            &format!("/rest/api/3/issue/{}/comment/{}", issue_key, comment_id),
            auth,
            None,
            Some(adf_body),
        ).await
    }

    pub async fn delete_comment(
        &self,
        issue_key: &str,
        comment_id: &str,
        auth: &Auth,
    ) -> Result<()> {
        tracing::info!(target: "jira", op = "delete_comment", issue_key = %issue_key, comment_id = %comment_id);

        self.make_request(
            reqwest::Method::DELETE,
            &format!("/rest/api/3/issue/{}/comment/{}", issue_key, comment_id),
            auth,
            None,
            None,
        ).await?;

        Ok(())
    }
}
