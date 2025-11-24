use std::collections::{HashMap, HashSet};

use super::context::JiraCtx;
use super::errors::log_err;

pub async fn compute_board_field_keys(
    ctx: &JiraCtx,
    issue: &jira_client::models::issue::IssueDetail,
    board_id: u64,
) -> Result<HashSet<String>, rmcp::ErrorData> {
    let cfg = ctx
        .client
        .get_board_configuration(board_id, &ctx.auth)
        .await
        .map_err(|e| log_err("get_issue", "jira_error", e.to_string()))?;

    let filter_id_val = cfg.get("filter").and_then(|o| o.get("id"));

    let mut sample = Vec::new();

    if let Some(fidv) = filter_id_val {
        let filter_id = match fidv {
            serde_json::Value::Number(n) => n.as_u64(),
            serde_json::Value::String(s) => s.parse::<u64>().ok(),
            _ => None,
        };

        if let Some(fid) = filter_id {
            if let Ok(filter) = ctx.client.get_filter(fid, &ctx.auth).await {
                if let Some(jql) = filter.get("jql").and_then(|v| v.as_str()) {
                    sample = ctx
                        .client
                        .search_issues_fields(jql, 100, &ctx.auth)
                        .await
                        .map_err(|e| log_err("get_issue", "jira_error", e.to_string()))?;
                }
            }
        }
    }

    if sample.is_empty() {
        sample = ctx
            .client
            .get_board_issues(board_id, 100, &ctx.auth)
            .await
            .map_err(|e| log_err("get_issue", "jira_error", e.to_string()))?;
    }

    let mut used: HashMap<String, u32> = HashMap::new();

    for iss in sample {
        if let Some(obj) = iss.fields.as_object() {
            for (k, v) in obj {
                if !v.is_null() {
                    *used.entry(k.clone()).or_default() += 1;
                }
            }
        }
    }

    let editmeta = ctx
        .client
        .get_issue_editmeta(&issue.key, &ctx.auth)
        .await
        .map_err(|e| log_err("get_issue", "jira_error", e.to_string()))?;

    let mut edit_keys: HashSet<String> = HashSet::new();

    if let Some(obj) = editmeta.get("fields").and_then(|v| v.as_object()) {
        edit_keys.extend(obj.keys().cloned());
    }

    let project_key = issue
        .fields
        .get("project")
        .and_then(|o| o.get("key"))
        .and_then(|s| s.as_str());

    let issue_type = issue
        .fields
        .get("issuetype")
        .and_then(|o| o.get("name"))
        .and_then(|s| s.as_str());

    let createmeta = ctx
        .client
        .get_createmeta(project_key, issue_type, &ctx.auth)
        .await
        .unwrap_or_else(|_| serde_json::json!({}));

    let mut create_keys: HashSet<String> = HashSet::new();

    if let Some(projects) = createmeta.get("projects").and_then(|v| v.as_array()) {
        for p in projects {
            if let Some(its) = p.get("issuetypes").and_then(|v| v.as_array()) {
                for it in its {
                    if let Some(fields) = it.get("fields").and_then(|v| v.as_object()) {
                        create_keys.extend(fields.keys().cloned());
                    }
                }
            }
        }
    }

    let estimation_field = cfg
        .get("estimation")
        .and_then(|e| e.get("field"))
        .and_then(|f| f.get("fieldId"))
        .and_then(|s| s.as_str())
        .map(|s| s.to_string());

    let mut keys: HashSet<String> = edit_keys.into_iter().collect();

    keys.extend(create_keys.into_iter());

    if let Some(f) = estimation_field {
        keys.insert(f);
    }

    for k in [
        "summary",
        "issuetype",
        "project",
        "status",
        "assignee",
        "labels",
        "components",
        "parent",
        "priority",
    ]
    .iter()
    {
        keys.insert((*k).to_string());
    }

    Ok(keys)
}
