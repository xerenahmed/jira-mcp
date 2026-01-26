use jira_client::error::JiraError;
use rmcp::ErrorData;
use serde_json::{json, Value};

pub fn log_err(tool: &str, code: &'static str, message: String) -> ErrorData {
    eprintln!("[tool:{}] {}: {}", tool, code, message);
    ErrorData::internal_error(code, Some(json!({"message": message})))
}

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

/// Context for error handling with suggestions and metadata
pub struct HandlerErrorContext {
    pub tool_name: &'static str,
    pub operation: &'static str,
    pub metadata: Value,
    suggestion_fn: Option<Box<dyn Fn(u16) -> Vec<String> + Send + Sync>>,
}

impl HandlerErrorContext {
    pub fn new(tool_name: &'static str, operation: &'static str) -> Self {
        Self {
            tool_name,
            operation,
            metadata: json!({}),
            suggestion_fn: None,
        }
    }

    pub fn with_metadata(mut self, key: &str, value: impl Into<Value>) -> Self {
        if let Some(obj) = self.metadata.as_object_mut() {
            obj.insert(key.to_string(), value.into());
        }
        self
    }

    pub fn with_suggestions<F>(mut self, f: F) -> Self
    where
        F: Fn(u16) -> Vec<String> + 'static + Send + Sync,
    {
        self.suggestion_fn = Some(Box::new(f));
        self
    }

    fn get_suggestions(&self, status_code: u16) -> Vec<String> {
        self.suggestion_fn
            .as_ref()
            .map(|f| f(status_code))
            .unwrap_or_default()
    }
}

/// Trait for converting errors to ErrorData with context
pub trait IntoMcpError {
    fn into_mcp_error(self, ctx: &HandlerErrorContext) -> ErrorData;
}

impl IntoMcpError for anyhow::Error {
    fn into_mcp_error(self, ctx: &HandlerErrorContext) -> ErrorData {
        tracing::error!(
            target: "mcp",
            tool = ctx.tool_name,
            error = %self,
            "Failed to {}", ctx.operation
        );

        if let Some(JiraError::ApiError { status_code, response }) = self.downcast_ref::<JiraError>() {
            let error_message = extract_error_message(response);
            let suggestions = ctx.get_suggestions(*status_code);

            let full_message = if suggestions.is_empty() {
                format!("Jira API Error ({}): {}", status_code, error_message)
            } else {
                format!(
                    "Jira API Error ({}): {}\n\nSuggestions:\n{}",
                    status_code,
                    error_message,
                    suggestions.iter().map(|s| format!("  - {}", s)).collect::<Vec<_>>().join("\n")
                )
            };

            let mut error_data = json!({
                "status_code": status_code,
                "jira_response": response,
                "suggestions": suggestions
            });

            if let Some(obj) = error_data.as_object_mut() {
                if let Some(meta_obj) = ctx.metadata.as_object() {
                    for (k, v) in meta_obj {
                        obj.insert(k.clone(), v.clone());
                    }
                }
            }

            return ErrorData::internal_error(full_message, Some(error_data));
        }

        ErrorData::internal_error(
            format!("Failed to {}: {}", ctx.operation, self),
            Some(ctx.metadata.clone()),
        )
    }
}

/// Extension trait for Result to convert errors with context
pub trait ResultMcpExt<T> {
    fn mcp_context(self, ctx: HandlerErrorContext) -> Result<T, ErrorData>;
}

impl<T> ResultMcpExt<T> for anyhow::Result<T> {
    fn mcp_context(self, ctx: HandlerErrorContext) -> Result<T, ErrorData> {
        self.map_err(|e| e.into_mcp_error(&ctx))
    }
}

#[macro_export]
macro_rules! error_ctx {
    ($tool:literal, $op:literal) => {
        $crate::errors::HandlerErrorContext::new($tool, $op)
    };
    ($tool:literal, $op:literal, $($key:ident = $val:expr),+ $(,)?) => {
        $crate::errors::HandlerErrorContext::new($tool, $op)
            $(.with_metadata(stringify!($key), $val))+
    };
}
