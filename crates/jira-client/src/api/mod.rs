use anyhow::Result;
use url::Url;

use crate::auth::{apply_auth, Auth};
use crate::error::JiraError;
pub mod issues;
pub mod projects;
pub mod boards;
pub mod users;

#[derive(Clone)]
pub struct ApiClient {
    pub(crate) base_url: Url,
    http: reqwest::Client,
}

impl ApiClient {
    pub fn new(base_url: Url, http: reqwest::Client) -> Self {
        Self { base_url, http }
    }

    async fn make_request(
        &self,
        method: reqwest::Method,
        path: &str,
        auth: &Auth,
        query_params: Option<Vec<(String, String)>>,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let mut url = self.base_url.join(path)?;

        if let Some(params) = query_params {
            url.query_pairs_mut().extend_pairs(params);
        }

        let mut req = self.http.request(method, url);

        if let Some(body) = body {
            req = req.json(&body);
        }

        let req = apply_auth(req, auth);
        let resp = req.send().await?;

        let status = resp.status();

        if !status.is_success() {
            let error_body = resp.text().await.unwrap_or_else(|_| "Failed to read error body".to_string());
            let error_json: serde_json::Value = serde_json::from_str(&error_body)
                .unwrap_or_else(|_| serde_json::json!({"raw_error": error_body}));

            return Err(JiraError::ApiError {
                status_code: status.as_u16(),
                response: error_json,
            }.into());
        }

        if status == reqwest::StatusCode::NO_CONTENT {
            return Ok(serde_json::json!({}));
        }

        let text = resp.text().await?;

        if text.is_empty() {
            return Ok(serde_json::json!({}));
        }

        Ok(serde_json::from_str(&text)?)
    }
}
