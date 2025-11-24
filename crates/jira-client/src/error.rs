use thiserror::Error;

#[derive(Debug, Error)]
pub enum JiraError {
    #[error("Jira API error (status {status_code}): {response}")]
    ApiError {
        status_code: u16,
        response: serde_json::Value,
    },
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
