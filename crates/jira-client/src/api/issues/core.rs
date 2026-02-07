use anyhow::Result;
use serde_json::Value;

use crate::auth::Auth;
use crate::models::{Issue, IssueDetail, IssueType};
use crate::utils::{adf_collect_text, normalize_whitespace, clean_value_recursive};
use crate::api::ApiClient;

use super::utils::{DEFAULT_PAGE_SIZE, extract_string_field, should_keep_field};

impl ApiClient {
    pub async fn get_createmeta(
        &self,
        project_key: Option<&str>,
        issue_type: Option<&str>,
        auth: &Auth,
    ) -> Result<Value> {
        tracing::info!(target: "jira", op = "get_createmeta", project_key = ?project_key, issue_type = ?issue_type);
        let mut query_params = vec![("expand".into(), "projects.issuetypes.fields".into())];

        if let Some(pk) = project_key {
            query_params.push(("projectKeys".into(), pk.into()));
        }

        if let Some(it) = issue_type {
            query_params.push(("issuetypeNames".into(), it.into()));
        }
        self.make_request(
            reqwest::Method::GET,
            "/rest/api/3/issue/createmeta",
            auth,
            Some(query_params),
            None,
        ).await
    }

    pub async fn create_issue(
        &self,
        payload: &Value,
        auth: &Auth,
    ) -> Result<(String, String)> {
        tracing::info!(target: "jira", op = "create_issue", payload = ?payload);
        let response = self.make_request(
            reqwest::Method::POST,
            "/rest/api/3/issue",
            auth,
            None,
            Some(payload.clone()),
        ).await?;
        let key = response
            .get("key")
            .and_then(|s| s.as_str())
            .ok_or_else(|| anyhow::anyhow!("missing key in response"))?
            .to_string();
        let browse = self.base_url.join(&format!("/browse/{}", key))?.to_string();
        Ok((key, browse))
    }

    pub async fn update_issue(
        &self,
        issue_key: &str,
        payload: &Value,
        auth: &Auth,
    ) -> Result<()> {
        tracing::info!(target: "jira", op = "update_issue", issue_key = %issue_key, payload = ?payload);
        self.make_request(
            reqwest::Method::PUT,
            &format!("/rest/api/3/issue/{}", issue_key),
            auth,
            None,
            Some(payload.clone()),
        ).await?;
        Ok(())
    }

    pub async fn get_issue_detail(&self, key: &str, auth: &Auth) -> Result<IssueDetail> {
        tracing::info!(target: "jira", op = "get_issue_detail", key = %key);
        let query_params = vec![
            ("fields".into(), "*all".into()),
            ("expand".into(), "schema,names".into()),
        ];
        let v = self.make_request(
            reqwest::Method::GET,
            &format!("/rest/api/3/issue/{}", key),
            auth,
            Some(query_params),
            None,
        ).await?;
        let names_map = v.get("names")
            .and_then(|n| n.as_object())
            .cloned()
            .unwrap_or_default();

        let schema = v.get("schema")
            .and_then(|s| s.as_object())
            .cloned()
            .unwrap_or_default();
        let mut fields = v.get("fields").cloned().unwrap_or(Value::Object(Default::default()));

        if let Some(obj) = fields.as_object_mut() {
            let null_keys: Vec<String> = obj
                .iter()
                .filter_map(|(k, v)| {
                    if v.is_null() && !should_keep_field(k) {
                        Some(k.clone())
                    } else {
                        None
                    }
                })
                .collect();
            for k in &null_keys {
                obj.remove(k);
            }
            obj.remove("comment");
            obj.remove("comments");
            let adf_like: Vec<String> = obj
                .iter()
                .filter_map(|(k, val)| {
                    if k == "description" {
                        return None;
                    }

                    if let Some(t) = val.get("type").and_then(|s| s.as_str()) {
                        if t == "doc" && val.get("content").map(|c| c.is_array()).unwrap_or(false) {
                            return Some(k.clone());
                        }
                    }
                    None
                })
                .collect();
            for k in &adf_like {
                obj.remove(k);
            }
        }

        if let Some(obj) = fields.as_object_mut() {
            if let Some(desc) = obj.get("description") {
                if desc.get("type").and_then(|x| x.as_str()) == Some("doc") {
                    let mut buf = String::new();
                    adf_collect_text(desc, &mut buf);
                    let text = normalize_whitespace(buf);
                    obj.insert("description".to_string(), Value::String(text));
                }
            }
        }
        let summary = fields
            .get("summary")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string());

        let flagged_field_id = names_map.iter()
            .find(|(_, name)| {
                name.as_str()
                    .map(|n| n.eq_ignore_ascii_case("flagged"))
                    .unwrap_or(false)
            })
            .map(|(id, _)| id.clone());

        let flagged = flagged_field_id.as_ref()
            .and_then(|id| fields.get(id))
            .map(|v| match v {
                Value::Array(arr) => !arr.is_empty(),
                Value::Null => false,
                _ => false,
            })
            .unwrap_or(false);

        let mapped_fields = if let Some(fields_obj) = fields.as_object() {
            let mut mapped = serde_json::Map::new();
            for (field_id, field_value) in fields_obj {
                let field_name = names_map.get(field_id)
                    .and_then(|v| v.as_str())
                    .or_else(|| {
                        schema.get(field_id)
                            .and_then(|s| s.get("name"))
                            .and_then(|n| n.as_str())
                    })
                    .unwrap_or(field_id);
                let cleaned_value = clean_value_recursive(field_value);
                let field_entry = Value::Object(serde_json::Map::from_iter([
                    ("name".to_string(), Value::String(field_name.to_string())),
                    ("value".to_string(), cleaned_value),
                ]));
                mapped.insert(field_id.clone(), field_entry);
            }
            Value::Object(mapped)
        } else {
            fields
        };
        let key = v
            .get("key")
            .and_then(|s| s.as_str())
            .unwrap_or(key)
            .to_string();
        let url = self.base_url.join(&format!("/browse/{}", key))?.to_string();
        Ok(IssueDetail {
            key,
            url,
            summary,
            flagged,
            fields: mapped_fields,
        })
    }

    pub async fn get_issue_editmeta(&self, key: &str, auth: &Auth) -> Result<Value> {
        tracing::info!(target: "jira", op = "get_issue_editmeta", key = %key);
        self.make_request(
            reqwest::Method::GET,
            &format!("/rest/api/3/issue/{}/editmeta", key),
            auth,
            None,
            None,
        ).await
    }

    pub async fn search_issues_fields(
        &self,
        jql: &str,
        limit: usize,
        auth: &Auth,
    ) -> Result<Vec<Issue>> {
        tracing::info!(target: "jira", op = "search_issues_fields", limit = limit);
        let mut start_at = 0usize;
        let mut collected: Vec<Issue> = Vec::new();
        let page_size = limit.min(DEFAULT_PAGE_SIZE);
        while collected.len() < limit {
            let query_params = vec![
                ("jql".into(), jql.to_string()),
                ("fields".into(), "*all".into()),
                ("startAt".into(), start_at.to_string()),
                ("maxResults".into(), page_size.to_string()),
            ];
            let v = self.make_request(
                reqwest::Method::GET,
                "/rest/api/3/search/jql",
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
                let key = extract_string_field(&it, "key", "");
                let fields = it.get("fields").cloned().unwrap_or(Value::Object(Default::default()));
                collected.push(Issue { key, fields });

                if collected.len() >= limit {
                    break;
                }
            }
            start_at += page_size;
        }
        Ok(collected)
    }

    pub async fn get_recent_issues(
        &self,
        project_key: Option<&str>,
        issue_type: Option<&str>,
        limit: usize,
        epic_field_key: Option<&str>,
        auth: &Auth,
    ) -> Result<Vec<Issue>> {
        tracing::info!(target: "jira", op = "get_recent_issues", project_key = ?project_key, issue_type = ?issue_type, limit = limit);
        let mut clauses = Vec::new();

        if let Some(pk) = project_key {
            clauses.push(format!("project = {}", pk));
        }

        if let Some(it) = issue_type {
            clauses.push(format!("issuetype = \"{}\"", it));
        }
        let jql = if clauses.is_empty() {
            "order by updated DESC".to_string()
        } else {
            format!("{} order by updated DESC", clauses.join(" AND "))
        };
        let mut fields = vec!["labels", "components", "assignee", "summary"];

        if let Some(epic) = epic_field_key {
            fields.push(epic);
        }
        let fields_param = fields.join(",");
        let mut start_at = 0usize;
        let mut collected: Vec<Issue> = Vec::new();
        let page_size = limit.min(DEFAULT_PAGE_SIZE);
        while collected.len() < limit {
            let query_params = vec![
                ("jql".into(), jql.clone()),
                ("fields".into(), fields_param.clone()),
                ("startAt".into(), start_at.to_string()),
                ("maxResults".into(), page_size.to_string()),
            ];
            let v = self.make_request(
                reqwest::Method::GET,
                "/rest/api/3/search/jql",
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
                let key = extract_string_field(&it, "key", "");
                let fields = it.get("fields").cloned().unwrap_or(Value::Object(Default::default()));
                collected.push(Issue { key, fields });

                if collected.len() >= limit {
                    break;
                }
            }
            start_at += page_size;
        }
        Ok(collected)
    }

    pub async fn search_issues(
        &self,
        jql: &str,
        fields: Option<&str>,
        limit: usize,
        start_at: Option<usize>,
        auth: &Auth,
    ) -> Result<Vec<Value>> {
        tracing::info!(target: "jira", op = "search_issues", jql = %jql, limit = limit, start_at = ?start_at, fields = ?fields);
        let fields_param = fields.unwrap_or("*all");
        let mut query_params = vec![
            ("jql".into(), jql.to_string()),
            ("fields".into(), fields_param.to_string()),
            ("maxResults".into(), limit.min(DEFAULT_PAGE_SIZE).to_string()),
        ];

        if let Some(offset) = start_at {
            query_params.push(("startAt".into(), offset.to_string()));
        }
        let v = self.make_request(
            reqwest::Method::GET,
            "/rest/api/3/search/jql",
            auth,
            Some(query_params),
            None,
        ).await?;
        let mut issues = v
            .get("issues")
            .and_then(|x| x.as_array())
            .cloned()
            .unwrap_or_default();
        for issue in &mut issues {
            if let Some(fields_obj) = issue.get_mut("fields").and_then(|f| f.as_object_mut()) {
                let null_keys: Vec<String> = fields_obj
                    .iter()
                    .filter_map(|(k, v)| {
                        if v.is_null() && !should_keep_field(k) {
                            Some(k.clone())
                        } else {
                            None
                        }
                    })
                    .collect();
                for k in null_keys {
                    fields_obj.remove(&k);
                }
                for (_, field_value) in fields_obj.iter_mut() {
                    *field_value = clean_value_recursive(field_value);
                }
            }
        }
        Ok(issues)
    }

    pub async fn list_issue_types(
        &self,
        project_key: Option<&str>,
        auth: &Auth,
    ) -> Result<Vec<IssueType>> {
        if let Some(pk) = project_key {
            let meta = self.get_createmeta(Some(pk), None, auth).await?;
            let mut out = Vec::new();

            if let Some(projects) = meta.get("projects").and_then(|v| v.as_array()) {
                for p in projects {
                    if p.get("key").and_then(|v| v.as_str()) != Some(pk) {
                        continue;
                    }

                    if let Some(its) = p.get("issuetypes").and_then(|v| v.as_array()) {
                        for it in its {
                            let id = extract_string_field(it, "id", "");
                            let name = extract_string_field(it, "name", "");
                            let description = it
                                .get("description")
                                .and_then(|s| s.as_str())
                                .map(|s| s.to_string());
                            let subtask =
                                it.get("subtask").and_then(|b| b.as_bool()).unwrap_or(false);
                            out.push(IssueType {
                                id,
                                name,
                                description,
                                subtask,
                            });
                        }
                    }
                }
            }
            Ok(out)
        } else {
            let v = self.make_request(
                reqwest::Method::GET,
                "/rest/api/3/issuetype",
                auth,
                None,
                None,
            ).await?;
            let mut out = Vec::new();

            if let Some(list) = v.as_array() {
                for it in list {
                    let id = extract_string_field(it, "id", "");
                    let name = extract_string_field(it, "name", "");
                    let description = it
                        .get("description")
                        .and_then(|s| s.as_str())
                        .map(|s| s.to_string());
                    let subtask = it.get("subtask").and_then(|b| b.as_bool()).unwrap_or(false);
                    out.push(IssueType {
                        id,
                        name,
                        description,
                        subtask,
                    });
                }
            }
            Ok(out)
        }
    }
}
