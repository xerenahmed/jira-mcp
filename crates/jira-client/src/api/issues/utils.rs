use serde_json::{json, Value};

pub const DEFAULT_PAGE_SIZE: usize = 100;

/// Visibility settings for a comment
#[derive(Debug, Clone)]
pub struct CommentVisibility {
    /// Type of visibility: "role" or "group"
    pub visibility_type: String,
    /// The role or group name (e.g., "Administrators", "jira-developers")
    pub value: String,
}

pub fn text_to_adf(text: &str) -> Value {
    json!({
        "type": "doc",
        "version": 1,
        "content": [
            {
                "type": "paragraph",
                "content": [
                    {
                        "type": "text",
                        "text": text
                    }
                ]
            }
        ]
    })
}

pub fn extract_string_field(obj: &Value, key: &str, default: &str) -> String {
    obj.get(key)
        .and_then(|s| s.as_str())
        .unwrap_or(default)
        .to_string()
}

pub fn should_keep_field(field_key: &str) -> bool {
    const ALWAYS_INCLUDE: &[&str] = &[
        "parent",
        "sprint",
        "epic",
        "subtasks",
        "issuelinks",
        "attachment",
        "assignee",
        "reporter",
    ];
    ALWAYS_INCLUDE.contains(&field_key)
}
