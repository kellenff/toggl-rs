use crate::workspace::Workspace;

use crate::Toggl;

#[derive(Deserialize, Debug, Serialize)]
pub struct UserJSON {
    pub fullname: String,
    pub workspaces: Vec<Workspace>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct InitResponse {
    since: i64,
    pub data: UserJSON,
}

impl Toggl {
    pub fn authenticate_api_token(api_token: &str) -> Result<Toggl, crate::error::TogglError> {
        let client = reqwest::Client::new();
        let ap = api_token.trim_end();
        let mut resp = client
            .get("https://www.toggl.com/api/v8/me")
            .basic_auth(ap, Some("api_token"))
            .send()?;
        if resp.status().is_success() {
            let init_response: InitResponse = resp.json()?;

            Ok(Toggl {
                api_token: ap.to_owned(),
                client,
                user: init_response.into(),
                clients: Vec::new(),
                projects: Vec::new(),
            })
        } else {
            Err(crate::error::TogglError::AuthError(
                format!(
                    "Authentication not succeded: Status {}, Text {}",
                    resp.status(),
                    resp.text().unwrap()
                )
                .to_owned(),
            ))
        }
    }
}
