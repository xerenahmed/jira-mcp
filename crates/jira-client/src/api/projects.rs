use anyhow::Result;

use crate::auth::Auth;
use crate::models::{Project, ProjectSummary};
use super::ApiClient;

impl ApiClient {
    pub async fn list_projects(&self, auth: &Auth) -> Result<Vec<Project>> {
        tracing::info!(target: "jira", op = "list_projects");

        let mut all_projects = Vec::new();
        let mut start_at = 0usize;
        const MAX_RESULTS: usize = 100;

        loop {
            let query_params = vec![
                ("expand".into(), "description,lead,projectCategory".into()),
                ("maxResults".into(), MAX_RESULTS.to_string()),
                ("startAt".into(), start_at.to_string()),
            ];

            let v = self.make_request(
                reqwest::Method::GET,
                "/rest/api/3/project/search",
                auth,
                Some(query_params),
                None,
            ).await?;

            if let Some(values) = v.get("values").and_then(|v| v.as_array()) {
                for project_val in values {
                    if let Ok(project) = serde_json::from_value::<Project>(project_val.clone()) {
                        all_projects.push(project);
                    }
                }


                let total = v.get("total").and_then(|t| t.as_u64()).unwrap_or(0) as usize;
                let fetched = all_projects.len();

                if fetched >= total || values.is_empty() {
                    break;
                }

                start_at += MAX_RESULTS;
            } else {
                break;
            }
        }

        Ok(all_projects)
    }

    pub async fn list_projects_summary(&self, auth: &Auth) -> Result<Vec<ProjectSummary>> {
        tracing::info!(target: "jira", op = "list_projects_summary");

        let mut all_projects = Vec::new();
        let mut start_at = 0usize;
        const MAX_RESULTS: usize = 100;

        loop {
            let query_params = vec![
                ("maxResults".into(), MAX_RESULTS.to_string()),
                ("startAt".into(), start_at.to_string()),
            ];

            let v = self.make_request(
                reqwest::Method::GET,
                "/rest/api/3/project/search",
                auth,
                Some(query_params),
                None,
            ).await?;

            if let Some(values) = v.get("values").and_then(|v| v.as_array()) {
                for project_val in values {
                    let key = project_val
                        .get("key")
                        .and_then(|s| s.as_str())
                        .unwrap_or("")
                        .to_string();

                    let name = project_val
                        .get("name")
                        .and_then(|s| s.as_str())
                        .unwrap_or("")
                        .to_string();

                    if !key.is_empty() && !name.is_empty() {
                        all_projects.push(ProjectSummary { key, name });
                    }
                }


                let total = v.get("total").and_then(|t| t.as_u64()).unwrap_or(0) as usize;
                let fetched = all_projects.len();

                if fetched >= total || values.is_empty() {
                    break;
                }

                start_at += MAX_RESULTS;
            } else {
                break;
            }
        }

        Ok(all_projects)
    }
}
