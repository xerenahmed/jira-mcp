/// Authentication for Jira API (always Basic auth with username + API token)
#[derive(Debug, Clone)]
pub struct Auth {
    pub username: String,
    pub token: String,
}

impl Auth {
    pub fn new(username: String, token: String) -> Self {
        Self { username, token }
    }
}

pub fn apply_auth(req: reqwest::RequestBuilder, auth: &Auth) -> reqwest::RequestBuilder {
    req.basic_auth(&auth.username, Some(&auth.token))
}
