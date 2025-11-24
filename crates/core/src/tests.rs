use serde_json::json;

use crate::fields_from_createmeta;
use crate::utils::process_jira_fields;

#[test]
fn test_fields_from_createmeta_exists() {

    let meta = serde_json::json!({});
    let _fields = fields_from_createmeta(&meta, None, None);
}

#[test]
fn test_components_with_name_objects() {
    let fields = json!({
        "components": [
            {"name": "Smart Recommender Analytics"}
        ]
    });

    let result = process_jira_fields(&fields);
    let components = result.get("components").unwrap();

    assert!(components.is_array());
    let arr = components.as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["name"], "Smart Recommender Analytics");
}

#[test]
fn test_components_with_id_objects() {
    let fields = json!({
        "components": [
            {"id": "214124"}
        ]
    });

    let result = process_jira_fields(&fields);
    let components = result.get("components").unwrap();

    assert!(components.is_array());
    let arr = components.as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["id"], "214124");
}

#[test]
fn test_components_with_id_numbers() {
    let fields = json!({
        "components": [
            {"id": 214124}
        ]
    });

    let result = process_jira_fields(&fields);
    let components = result.get("components").unwrap();

    assert!(components.is_array());
    let arr = components.as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["id"], 214124);
}

#[test]
fn test_components_with_string_array() {
    let fields = json!({
        "components": ["Component1", "Component2"]
    });

    let result = process_jira_fields(&fields);
    let components = result.get("components").unwrap();

    assert!(components.is_array());
    let arr = components.as_array().unwrap();
    assert_eq!(arr.len(), 2);
    assert_eq!(arr[0]["name"], "Component1");
    assert_eq!(arr[1]["name"], "Component2");
}

#[test]
fn test_components_empty_array() {
    let fields = json!({
        "components": []
    });

    let result = process_jira_fields(&fields);
    let components = result.get("components").unwrap();

    assert!(components.is_array());
    let arr = components.as_array().unwrap();
    assert_eq!(arr.len(), 0);
}

#[test]
fn test_components_mixed_fields() {
    let fields = json!({
        "components": [
            {"name": "Component1", "id": "123"}
        ]
    });

    let result = process_jira_fields(&fields);
    let components = result.get("components").unwrap();

    assert!(components.is_array());
    let arr = components.as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["name"], "Component1");
    assert_eq!(arr[0]["id"], "123");
}
