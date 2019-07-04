use crate::Toggl;

static AUTH_URL: &'static str = "https://www.toggl.com/api/v8/me";

impl std::convert::From<reqwest::Error> for crate::error::TogglError {
    fn from(e: reqwest::Error) -> crate::error::TogglError {
        crate::error::TogglError::ReqwestError(e)
    }
}

impl std::convert::From<reqwest::header::InvalidHeaderValue> for crate::error::TogglError {
    fn from(e: reqwest::header::InvalidHeaderValue) -> crate::error::TogglError {
        crate::error::TogglError::AuthError("Could not parse Authentication api_token".to_owned())
    }
}


#[derive(Deserialize, Debug, Serialize)]
struct InitResponse {
    since: i64,
    data: crate::user::User,
}

pub fn init(api_token: &str) -> Result<Toggl, crate::error::TogglError> {
    let client = reqwest::Client::new();
    let mut resp = client.get(AUTH_URL)
        .basic_auth(api_token, Some("api_token"))
        .send()?;
    if resp.status().is_success() {
        //println!("{:?}", resp.text());
        let init_response: InitResponse = resp.json()?;

        Ok(Toggl {
            api_token: api_token.to_owned(),
            client,
            user: init_response.data,
            workspaces: None,
            projects: None,
        })
    } else {
        Err(crate::error::TogglError::AuthError(format!("Authentication not succeded: Status {}, Text {}", resp.status(), resp.text().unwrap()).to_owned()))
    }
}