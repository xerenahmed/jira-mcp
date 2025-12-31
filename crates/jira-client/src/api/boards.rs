use anyhow::Result;
use serde_json::Value;

use crate::auth::Auth;
use crate::models::{Board, Issue};
use super::ApiClient;

impl ApiClient {
    pub async fn get_board_configuration(
        &self,
        board_id: u64,
        auth: &Auth,
    ) -> Result<Value> {
        tracing::info!(target: "jira", op = "get_board_configuration", board_id = board_id);

        self.make_request(
            reqwest::Method::GET,
            &format!("/rest/agile/1.0/board/{}/configuration", board_id),
            auth,
            None,
            None,
        ).await
    }

    pub async fn get_filter(&self, filter_id: u64, auth: &Auth) -> Result<Value> {
        tracing::info!(target: "jira", op = "get_filter", filter_id = filter_id);

        self.make_request(
            reqwest::Method::GET,
            &format!("/rest/api/3/filter/{}", filter_id),
            auth,
            None,
            None,
        ).await
    }

    pub async fn get_board_issues(
        &self,
        board_id: u64,
        limit: usize,
        auth: &Auth,
    ) -> Result<Vec<Issue>> {
        tracing::info!(target: "jira", op = "get_board_issues", board_id = board_id, limit = limit);

        let mut start_at = 0usize;
        let mut out: Vec<Issue> = Vec::new();
        let page_size = limit.min(100);

        while out.len() < limit {
            let query_params = vec![
                ("fields".into(), "*all".into()),
                ("startAt".into(), start_at.to_string()),
                ("maxResults".into(), page_size.to_string()),
            ];

            let v = self.make_request(
                reqwest::Method::GET,
                &format!("/rest/agile/1.0/board/{}/issue", board_id),
                auth,
                Some(query_params),
                None,
            ).await?;

            let issues = v
                .get("issues")
                .and_then(|x| x.as_array())
                .cloned()
                .unwrap_or_default();

            if issues.is_empty() {
                break;
            }

            for it in issues {
                let key = it
                    .get("key")
                    .and_then(|s| s.as_str())
                    .unwrap_or("")
                    .to_string();

                let fields = it.get("fields").cloned().unwrap_or(Value::Object(Default::default()));

                out.push(Issue { key, fields });

                if out.len() >= limit {
                    break;
                }
            }

            start_at += page_size;
        }

        Ok(out)
    }

    pub async fn list_boards(
        &self,
        project_key: &str,
        auth: &Auth,
    ) -> Result<Vec<Board>> {
        tracing::info!(target: "jira", op = "list_boards", project_key = %project_key);

        let mut all_boards = Vec::new();
        let mut start_at = 0usize;
        const MAX_RESULTS: usize = 50;

        loop {
            let query_params = vec![
                ("projectKeyOrId".into(), project_key.to_string()),
                ("maxResults".into(), MAX_RESULTS.to_string()),
                ("startAt".into(), start_at.to_string()),
            ];

            let v = self.make_request(
                reqwest::Method::GET,
                "/rest/agile/1.0/board",
                auth,
                Some(query_params),
                None,
            ).await?;

            if let Some(values) = v.get("values").and_then(|v| v.as_array()) {
                for board_val in values {
                    if let Ok(board) = serde_json::from_value::<Board>(board_val.clone()) {
                        all_boards.push(board);
                    }
                }


                let total = v.get("total").and_then(|t| t.as_u64()).unwrap_or(0) as usize;
                let fetched = all_boards.len();

                if fetched >= total || values.is_empty() {
                    break;
                }

                start_at += MAX_RESULTS;
            } else {
                break;
            }
        }

        Ok(all_boards)
    }

    pub async fn move_issues_to_backlog(
        &self,
        issue_keys: &[String],
        auth: &Auth,
    ) -> Result<()> {
        tracing::info!(
            target: "jira",
            op = "move_issues_to_backlog",
            issues = ?issue_keys
        );

        let body = serde_json::json!({
            "issues": issue_keys
        });

        self.make_request(
            reqwest::Method::POST,
            "/rest/agile/1.0/backlog/issue",
            auth,
            None,
            Some(body),
        ).await?;

        Ok(())
    }
}
