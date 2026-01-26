use anyhow::Result;
use serde_json::Value;

use crate::auth::Auth;
use crate::api::ApiClient;

impl ApiClient {
    pub async fn get_transitions(
        &self,
        issue_key: &str,
        expand: Option<&str>,
        auth: &Auth,
    ) -> Result<Value> {
        tracing::info!(target: "jira", op = "get_transitions", issue_key = %issue_key, expand = ?expand);
        let mut query_params = Vec::new();
        if let Some(exp) = expand {
            query_params.push(("expand".into(), exp.into()));
        }
        let params = if query_params.is_empty() {
            None
        } else {
            Some(query_params)
        };
        self.make_request(
            reqwest::Method::GET,
            &format!("/rest/api/3/issue/{}/transitions", issue_key),
            auth,
            params,
            None,
        ).await
    }

    pub async fn transition_issue(
        &self,
        issue_key: &str,
        transition_id: &str,
        fields: Option<&serde_json::Value>,
        comment: Option<&str>,
        auth: &Auth,
    ) -> Result<()> {
        tracing::info!(target: "jira", op = "transition_issue", issue_key = %issue_key, transition_id = %transition_id);

        let mut payload = serde_json::json!({
            "transition": {
                "id": transition_id
            }
        });

        if let Some(fields_val) = fields {
            if let Some(obj) = payload.as_object_mut() {
                obj.insert("fields".to_string(), fields_val.clone());
            }
        }

        if let Some(comment_text) = comment {
            let comment_adf = serde_json::json!({
                "type": "doc",
                "version": 1,
                "content": [{
                    "type": "paragraph",
                    "content": [{
                        "type": "text",
                        "text": comment_text
                    }]
                }]
            });

            if let Some(obj) = payload.as_object_mut() {
                obj.insert("update".to_string(), serde_json::json!({
                    "comment": [{ "add": { "body": comment_adf } }]
                }));
            }
        }

        self.make_request(
            reqwest::Method::POST,
            &format!("/rest/api/3/issue/{}/transitions", issue_key),
            auth,
            None,
            Some(payload),
        ).await?;

        Ok(())
    }

    pub async fn assign_issue(
        &self,
        issue_key: &str,
        account_id: Option<&str>,
        auth: &Auth,
    ) -> Result<()> {
        tracing::info!(target: "jira", op = "assign_issue", issue_key = %issue_key, account_id = ?account_id);

        let body = serde_json::json!({
            "accountId": account_id
        });

        self.make_request(
            reqwest::Method::PUT,
            &format!("/rest/api/3/issue/{}/assignee", issue_key),
            auth,
            None,
            Some(body),
        ).await?;

        Ok(())
    }
}
