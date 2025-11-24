pub fn log_err(tool: &str, code: &'static str, message: String) -> rmcp::ErrorData {
    eprintln!("[tool:{}] {}: {}", tool, code, message);
    rmcp::ErrorData::internal_error(code, Some(serde_json::json!({"message": message})))
}
