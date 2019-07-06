
use crate::user::User;
use crate::workspace::Workspace;

use crate::Toggl;
use std::rc::Rc;
impl std::convert::From<reqwest::Error> for crate::error::TogglError {
    fn from(e: reqwest::Error) -> crate::error::TogglError {
        crate::error::TogglError::ReqwestError(e)
    }
}

impl std::convert::From<reqwest::header::InvalidHeaderValue> for crate::error::TogglError {
    fn from(_e: reqwest::header::InvalidHeaderValue) -> crate::error::TogglError {
        crate::error::TogglError::AuthError("Could not parse Authentication api_token".to_owned())
    }
}


#[derive(Deserialize, Debug, Serialize)]
struct UserJSON {
    fullname: String,
    workspaces: Vec<Workspace>,
}

#[derive(Deserialize, Debug, Serialize)]
struct InitResponse {
    since: i64,
    data: UserJSON,
}

pub fn init(api_token: &str) -> Result<Toggl, crate::error::TogglError> {
    let client = reqwest::Client::new();
    let ap = api_token.trim_end();
    let mut resp = client
        .get("https://www.toggl.com/api/v8/me")
        .basic_auth(ap, Some("api_token"))
        .send()?;
    if resp.status().is_success() {
        let init_response: InitResponse = resp.json()?;
        let user = User {
            fullname: init_response.data.fullname,
            workspaces: init_response
                .data
                .workspaces
                .into_iter()
                .map(Rc::new)
                .collect(),
        };

        Ok(Toggl {
            api_token: ap.to_owned(),
            client,
            user,
            projects: None,
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