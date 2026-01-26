const FIELDS_TO_REMOVE: &[&str] = &[
    "avatarUrls",
    "iconUrl",
    "self",
    "accountType",
    "active",
    "content",
    "thumbnail",
    "isLast",
    "startAt",
    "maxResults",
    "total",
    "attachment",
];

pub fn clean_value_recursive(value: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(obj) => {
            let mut cleaned = serde_json::Map::new();
            for (key, val) in obj.iter() {
                if !FIELDS_TO_REMOVE.contains(&key.as_str()) {
                    cleaned.insert(key.clone(), clean_value_recursive(val));
                }
            }
            serde_json::Value::Object(cleaned)
        }
        serde_json::Value::Array(arr) => {
            serde_json::Value::Array(arr.iter().map(clean_value_recursive).collect())
        }
        other => other.clone(),
    }
}
