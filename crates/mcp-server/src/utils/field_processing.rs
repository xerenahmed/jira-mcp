use serde_json::Value;

use crate::models::FieldDef;

pub fn process_jira_fields(fields: &serde_json::Value) -> serde_json::Map<String, Value> {
    if let Some(s) = fields.as_str() {
        tracing::warn!(
            target: "core",
            "Received fields as JSON string instead of object, parsing automatically"
        );

        match serde_json::from_str::<serde_json::Value>(s) {
            Ok(parsed) => {
                return process_jira_fields(&parsed);
            }
            Err(e) => {
                tracing::error!(target: "core", error = %e, "Failed to parse fields string as JSON");

                return serde_json::Map::new();
            }
        }
    }

    let mut processed_fields = serde_json::Map::new();

    let Some(fields_obj) = fields.as_object() else {
        tracing::error!(target: "core", "Fields is neither an object nor a string");

        return processed_fields;
    };

    for (field_name, field_value) in fields_obj {
        let final_value = match field_name.as_str() {
            "description" if field_value.is_string() => {
                let text = field_value.as_str().unwrap_or("");
                serde_json::json!({
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
            "priority" if field_value.is_string() => {
                let priority = field_value.as_str().unwrap_or("");
                let priority_name = priority.split_whitespace().last().unwrap_or(priority);
                serde_json::Value::String(priority_name.to_string())
            }
            "components" if field_value.is_array() => {
                let arr = field_value.as_array().unwrap();


                match arr.first() {
                    Some(first) if first.is_object() => {

                        field_value.clone()
                    }
                    Some(_) => {

                        let comps: Vec<Value> = arr
                            .iter()
                            .filter_map(|v| v.as_str())
                            .map(|name| serde_json::json!({"name": name}))
                            .collect();
                        serde_json::Value::Array(comps)
                    }
                    None => {

                        field_value.clone()
                    }
                }
            }
            "fixVersions" if field_value.is_array() => {
                let arr = field_value.as_array().unwrap();

                match arr.first() {
                    Some(first) if first.is_object() => {

                        field_value.clone()
                    }
                    Some(_) => {

                        let vers: Vec<Value> = arr
                            .iter()
                            .filter_map(|v| v.as_str())
                            .map(|name| serde_json::json!({"name": name}))
                            .collect();
                        serde_json::Value::Array(vers)
                    }
                    None => {

                        field_value.clone()
                    }
                }
            }
            _ => field_value.clone(),
        };

        processed_fields.insert(field_name.clone(), final_value);
    }

    processed_fields
}

pub fn fields_from_createmeta(
    meta: &Value,
    project_key: Option<&str>,
    issue_type: Option<&str>,
) -> Vec<FieldDef> {
    let mut out = Vec::new();

    if let Some(projects) = meta.get("projects").and_then(|v| v.as_array()) {
        'p: for p in projects {
            if let Some(pk) = project_key {
                if p.get("key").and_then(|v| v.as_str()) != Some(pk) {
                    continue 'p;
                }
            }

            if let Some(its) = p.get("issuetypes").and_then(|v| v.as_array()) {
                for it in its {
                    if let Some(itn) = issue_type {
                        if it.get("name").and_then(|v| v.as_str()) != Some(itn) {
                            continue;
                        }
                    }

                    if let Some(fields) = it.get("fields").and_then(|v| v.as_object()) {
                        for (fid, fdef) in fields.iter() {
                            let name = fdef
                                .get("name")
                                .and_then(|v| v.as_str())
                                .unwrap_or(fid)
                                .to_string();

                            let required = fdef
                                .get("required")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false);

                            let schema =
                                fdef.get("schema").cloned().unwrap_or(serde_json::json!({}));

                            let allowed_values = fdef
                                .get("allowedValues")
                                .cloned()
                                .unwrap_or(serde_json::json!([]));

                            out.push(FieldDef {
                                id: fid.clone(),
                                name,
                                required,
                                schema,
                                allowed_values,
                            });
                        }
                    }
                }
            }
        }
    }

    out
}
