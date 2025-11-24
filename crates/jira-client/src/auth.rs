#[derive(Debug, Clone)]
pub enum Auth {
    None,
    Basic { username: String, token: String },
    Bearer { token: String },
}

pub fn apply_auth(req: reqwest::RequestBuilder, auth: &Auth) -> reqwest::RequestBuilder {
    match auth {
        Auth::None => req,
        Auth::Basic { username, token } => req.basic_auth(username.clone(), Some(token.clone())),
        Auth::Bearer { token } => req.bearer_auth(token.clone()),
    }
}
