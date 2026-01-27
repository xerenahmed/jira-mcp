use jira_client::error::JiraError;
use rmcp::ErrorData;
use serde_json::{json, Value};

pub fn extract_error_message(response: &Value) -> String {
    if let Some(error_messages) = response.get("errorMessages").and_then(|m| m.as_array()) {
        if !error_messages.is_empty() {
            return error_messages
                .iter()
                .filter_map(|m| m.as_str())
                .collect::<Vec<_>>()
                .join("; ");
        }
    }

    if let Some(errors) = response.get("errors").and_then(|e| e.as_object()) {
        let mut messages = Vec::new();
        for (field, message) in errors {
            if let Some(msg) = message.as_str() {
                messages.push(format!("{}: {}", field, msg));
            }
        }

        if !messages.is_empty() {
            return messages.join("; ");
        }
    }

    if let Some(message) = response.get("message").and_then(|m| m.as_str()) {
        return message.to_string();
    }
    response.to_string()
}

pub struct ErrorContext {
    pub status_code: u16,
    pub jira_response: Value,
    pub suggestions: Vec<String>,
    pub metadata: Option<Value>,
}

pub fn get_jql_suggestions(jql: &str, status_code: u16) -> Vec<String> {
    let mut suggestions = Vec::new();

    match status_code {
        400 => {
            suggestions.push("Check your JQL syntax for common errors".to_string());

            if jql.contains("=") && !jql.contains('\'') && !jql.contains('"') {
                suggestions.push("String values should be quoted: project = PROJ instead of project = PROJ".to_string());
            }

            if jql.contains("  ") {
                suggestions.push("Remove double spaces in JQL".to_string());
            }
            suggestions.push("Try a simpler query like: project = PROJ".to_string());
        }
        401 => {
            suggestions.push("Check your authentication credentials".to_string());
            suggestions.push("Ensure your API token is valid and has proper permissions".to_string());
        }
        403 => {
            suggestions.push("You don't have permission to search in this project".to_string());
            suggestions.push("Check if the project exists and you have browse permissions".to_string());
        }
        429 => {
            suggestions.push("Rate limit exceeded. Try again in a few seconds".to_string());
            suggestions.push("Consider using a more specific query to reduce results".to_string());
        }
        500 => {
            suggestions.push("Jira server error. Try again later".to_string());
            suggestions.push("Consider using a simpler query".to_string());
        }
        _ => {
            suggestions.push("Check your JQL syntax and permissions".to_string());
            suggestions.push("Try verifying project keys and field names".to_string());
        }
    }
    suggestions
}

pub fn get_create_suggestions(
    project_key: &Option<String>,
    issue_type: &Option<String>,
    status_code: u16,
) -> Vec<String> {
    let mut suggestions = Vec::new();

    match status_code {
        400 => {
            suggestions.push("Check if all required fields are provided".to_string());

            if let Some(pk) = project_key {
                suggestions.push(format!("Verify project key '{}' exists", pk));
            }

            if let Some(it) = issue_type {
                suggestions.push(format!("Verify issue type '{}' exists in project", it));
            }
            suggestions.push("Check field values match expected formats".to_string());
        }
        403 => {
            suggestions.push("You don't have permission to create issues in this project".to_string());
            suggestions.push("Ensure you have the 'Create Issues' permission".to_string());
        }
        404 => {
            suggestions.push("Project or issue type not found".to_string());
            suggestions.push("Verify the project key and issue type are correct".to_string());
        }
        _ => {
            suggestions.push("Check your input and permissions".to_string());
            suggestions.push("Try with minimal required fields first".to_string());
        }
    }
    suggestions
}

pub fn get_update_suggestions(issue_key: &str, status_code: u16) -> Vec<String> {
    let mut suggestions = Vec::new();

    match status_code {
        400 => {
            suggestions.push("Check field values and formats".to_string());
            suggestions.push("Verify the issue is not in a locked status".to_string());
        }
        403 => {
            suggestions.push("You don't have permission to edit this issue".to_string());
            suggestions.push("Ensure you have the 'Edit Issues' permission".to_string());
            suggestions.push("Check if the issue is in a transition that allows editing".to_string());
        }
        404 => {
            suggestions.push(format!("Issue '{}' not found", issue_key));
            suggestions.push("Verify the issue key is correct".to_string());
        }
        _ => {
            suggestions.push("Check your field values and permissions".to_string());
            suggestions.push("Ensure the issue is not in a read-only state".to_string());
        }
    }
    suggestions
}

pub fn handle_api_error(err: &JiraError, context: ErrorContext) -> ErrorData {
    if let JiraError::ApiError { status_code, response } = err {
        let error_message = extract_error_message(response);
        let full_message = if context.suggestions.is_empty() {
            format!("Jira API Error ({}): {}", status_code, error_message)
        } else {
            format!(
                "Jira API Error ({}): {}\n\nSuggestions:\n{}",
                status_code,
                error_message,
                context
                    .suggestions
                    .iter()
                    .map(|s| format!("  - {}", s))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        };
        let mut error_data = json!({
            "status_code": status_code,
            "jira_response": response,
            "suggestions": context.suggestions
        });

        if let Some(metadata) = context.metadata {
            if let Some(obj) = error_data.as_object_mut() {
                obj.insert("metadata".to_string(), metadata);
            }
        }
        ErrorData::internal_error(full_message, Some(error_data))
    } else {
        ErrorData::internal_error("Unknown Jira error", None)
    }
}
