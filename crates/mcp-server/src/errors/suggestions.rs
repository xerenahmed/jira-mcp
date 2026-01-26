pub fn get_jql_suggestions(jql: &str, status_code: u16) -> Vec<String> {
    let mut suggestions = Vec::new();

    match status_code {
        400 => {
            suggestions.push("Check your JQL syntax for common errors".to_string());

            if jql.contains("=") && !jql.contains('\'') && !jql.contains('"') {
                suggestions.push("String values should be quoted: project = 'PROJ' instead of project = PROJ".to_string());
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
    project_key: Option<&str>,
    issue_type: Option<&str>,
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

pub fn get_transition_suggestions(issue_key: &str, status_code: u16) -> Vec<String> {
    let mut suggestions = Vec::new();

    match status_code {
        400 => {
            suggestions.push("Check if the transition requires additional fields".to_string());
            suggestions.push("Verify the transition ID is correct".to_string());
        }
        403 => {
            suggestions.push("You don't have permission to transition this issue".to_string());
            suggestions.push("Check if you have the 'Transition Issues' permission".to_string());
        }
        404 => {
            suggestions.push(format!("Issue '{}' or transition not found", issue_key));
            suggestions.push("Use get_transitions to see available transitions".to_string());
        }
        _ => {
            suggestions.push("Check your transition ID and permissions".to_string());
        }
    }
    suggestions
}

pub fn get_comment_suggestions(issue_key: &str, status_code: u16) -> Vec<String> {
    let mut suggestions = Vec::new();

    match status_code {
        400 => {
            suggestions.push("Check if comment body is valid".to_string());
        }
        403 => {
            suggestions.push("You don't have permission to comment on this issue".to_string());
            suggestions.push("Check if you have the 'Add Comments' permission".to_string());
        }
        404 => {
            suggestions.push(format!("Issue '{}' or comment not found", issue_key));
            suggestions.push("Verify the issue key and comment ID are correct".to_string());
        }
        _ => {
            suggestions.push("Check your permissions for this issue".to_string());
        }
    }
    suggestions
}

pub fn get_watcher_suggestions(issue_key: &str, status_code: u16) -> Vec<String> {
    let mut suggestions = Vec::new();

    match status_code {
        403 => {
            suggestions.push("You don't have permission to manage watchers".to_string());
            suggestions.push("Check if you have the 'Manage Watchers' permission".to_string());
        }
        404 => {
            suggestions.push(format!("Issue '{}' or user not found", issue_key));
            suggestions.push("Verify the issue key and account ID are correct".to_string());
        }
        _ => {
            suggestions.push("Check your permissions and input values".to_string());
        }
    }
    suggestions
}

pub fn get_label_suggestions(issue_key: &str, status_code: u16) -> Vec<String> {
    let mut suggestions = Vec::new();

    match status_code {
        400 => {
            suggestions.push("Check if label names are valid".to_string());
            suggestions.push("Labels cannot contain spaces".to_string());
        }
        403 => {
            suggestions.push("You don't have permission to edit labels".to_string());
            suggestions.push("Check if you have the 'Edit Issues' permission".to_string());
        }
        404 => {
            suggestions.push(format!("Issue '{}' not found", issue_key));
            suggestions.push("Verify the issue key is correct".to_string());
        }
        _ => {
            suggestions.push("Check your permissions and label format".to_string());
        }
    }
    suggestions
}

pub fn get_link_suggestions(status_code: u16) -> Vec<String> {
    let mut suggestions = Vec::new();

    match status_code {
        400 => {
            suggestions.push("Check if the link type name is correct".to_string());
            suggestions.push("Use list_link_types to see available link types".to_string());
        }
        403 => {
            suggestions.push("You don't have permission to link issues".to_string());
            suggestions.push("Check if you have the 'Link Issues' permission".to_string());
        }
        404 => {
            suggestions.push("One or both issues not found".to_string());
            suggestions.push("Verify both issue keys are correct".to_string());
        }
        _ => {
            suggestions.push("Check your permissions and issue keys".to_string());
        }
    }
    suggestions
}
