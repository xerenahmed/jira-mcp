use anyhow::Result;
use serde_json::Value;

use crate::auth::Auth;
use crate::api::ApiClient;

impl ApiClient {
    pub async fn link_issues(
        &self,
        inward_issue_key: &str,
        outward_issue_key: &str,
        link_type: &str,
        auth: &Auth,
    ) -> Result<()> {
        tracing::info!(
            target: "jira",
            op = "link_issues",
            inward = %inward_issue_key,
            outward = %outward_issue_key,
            link_type = %link_type
        );

        let body = serde_json::json!({
            "type": {
                "name": link_type
            },
            "inwardIssue": {
                "key": inward_issue_key
            },
            "outwardIssue": {
                "key": outward_issue_key
            }
        });

        self.make_request(
            reqwest::Method::POST,
            "/rest/api/3/issueLink",
            auth,
            None,
            Some(body),
        ).await?;

        Ok(())
    }

    pub async fn delete_issue_link(
        &self,
        link_id: &str,
        auth: &Auth,
    ) -> Result<()> {
        tracing::info!(target: "jira", op = "delete_issue_link", link_id = %link_id);

        self.make_request(
            reqwest::Method::DELETE,
            &format!("/rest/api/3/issueLink/{}", link_id),
            auth,
            None,
            None,
        ).await?;

        Ok(())
    }

    pub async fn list_link_types(&self, auth: &Auth) -> Result<Vec<Value>> {
        tracing::info!(target: "jira", op = "list_link_types");

        let response: Value = self.make_request(
            reqwest::Method::GET,
            "/rest/api/3/issueLinkType",
            auth,
            None,
            None,
        ).await?;

        let link_types = response
            .get("issueLinkTypes")
            .and_then(|lt| lt.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|lt| {
                        serde_json::json!({
                            "id": lt.get("id").and_then(|v| v.as_str()).unwrap_or(""),
                            "name": lt.get("name").and_then(|v| v.as_str()).unwrap_or(""),
                            "inward": lt.get("inward").and_then(|v| v.as_str()).unwrap_or(""),
                            "outward": lt.get("outward").and_then(|v| v.as_str()).unwrap_or("")
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(link_types)
    }
}
