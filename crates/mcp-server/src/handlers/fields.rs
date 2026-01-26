use std::collections::HashSet;

use anyhow::Result;
use jira_client::utils::clean_value_recursive;
use rmcp::model::CallToolResult;

use crate::context::JiraCtx;
use crate::errors::log_err;
use crate::models::{GetFieldDetailsInput, ListFieldsInput};
use crate::utils::fields_from_createmeta;

pub async fn list_fields_handler(
    input: ListFieldsInput,
    ctx: &JiraCtx,
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(target: "mcp", tool = "list_fields", project = %input.project_key, issuetype = %input.issue_type);

    let meta = ctx
        .client
        .get_createmeta(Some(&input.project_key), Some(&input.issue_type), &ctx.auth)
        .await
        .map_err(|e| log_err("list_fields", "jira_error", e.to_string()))?;

    let mut fields = fields_from_createmeta(&meta, Some(&input.project_key), Some(&input.issue_type));

    if input.include_required_only {
        fields.retain(|f| f.required);
    }

    if let Some(ref field_names) = input.field_names {
        let name_set: HashSet<String> = field_names.iter().map(|n| n.to_lowercase()).collect();
        fields.retain(|f|
            name_set.contains(&f.id.to_lowercase()) ||
            name_set.contains(&f.name.to_lowercase())
        );
    }

    if let Some(ref field_types) = input.field_types {
        let type_set: HashSet<String> = field_types.iter().map(|t| t.to_lowercase()).collect();
        fields.retain(|f| {
            let field_type = f.schema
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_lowercase();
            type_set.contains(&field_type)
        });
    }

    let mut result = serde_json::Map::new();

    for f in fields {
        let field_type = f.schema
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        let required_status = if f.required { "required" } else { "optional" };

        result.insert(
            f.id.clone(),
            serde_json::Value::String(format!("{}|{}|{}", f.name, field_type, required_status)),
        );
    }

    Ok(CallToolResult::structured(serde_json::json!({
        "fields": serde_json::Value::Object(result)
    })))
}

pub async fn get_field_details_handler(
    input: GetFieldDetailsInput,
    ctx: &JiraCtx,
) -> Result<CallToolResult, rmcp::ErrorData> {
    tracing::info!(target: "mcp", tool = "get_field_details", project = %input.project_key, issuetype = %input.issue_type, count = input.field_ids.len());

    let meta = ctx
        .client
        .get_createmeta(Some(&input.project_key), Some(&input.issue_type), &ctx.auth)
        .await
        .map_err(|e| log_err("get_field_details", "jira_error", e.to_string()))?;

    let all_fields = fields_from_createmeta(&meta, Some(&input.project_key), Some(&input.issue_type));

    let requested_set: HashSet<String> = input.field_ids
        .iter()
        .map(|id| id.to_lowercase())
        .collect();

    let mut result = serde_json::Map::new();

    for field in all_fields {
        if requested_set.contains(&field.id.to_lowercase()) {
            let mut field_detail = serde_json::Map::new();
            field_detail.insert("name".to_string(), serde_json::Value::String(field.name));
            field_detail.insert("type".to_string(),
                serde_json::Value::String(
                    field.schema
                        .get("type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown")
                        .to_string()
                )
            );
            field_detail.insert("required".to_string(), serde_json::Value::Bool(field.required));

            if !field.allowed_values.is_null() && !field.allowed_values.as_array().map(|a| a.is_empty()).unwrap_or(false) {
                field_detail.insert("allowed_values".to_string(), clean_value_recursive(&field.allowed_values));
            }

            if let Some(schema_obj) = field.schema.as_object() {
                let mut schema_copy = schema_obj.clone();
                schema_copy.remove("type");

                if !schema_copy.is_empty() {
                    field_detail.insert("schema".to_string(), serde_json::Value::Object(schema_copy));
                }
            }

            result.insert(field.id, serde_json::Value::Object(field_detail));
        }
    }

    Ok(CallToolResult::structured(serde_json::json!({
        "fields": serde_json::Value::Object(result)
    })))
}
