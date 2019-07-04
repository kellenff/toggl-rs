static AUTH_URL: &'static str = "https://www.toggl.com/api/v8/me";

#[derive(Debug)]
pub struct Toggl {
    api_token: String,
    client: reqwest::Client,
}

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

pub fn init(api_token: &str) -> Result<Toggl, crate::error::TogglError> {
    let client = reqwest::Client::new();
    let mut resp = client.get(AUTH_URL)
        .basic_auth(api_token, Some("api_token"))
        .send()?;
    if resp.status().is_success() {
        Ok(Toggl {
            api_token: api_token.to_owned(),
            client,
        })
    } else {
        Err(crate::error::TogglError::AuthError(format!("Authentication not succeded: Status {}, Text {}", resp.status(), resp.text().unwrap()).to_owned()))
    }
}


//fn load_cookie(api_token) -> Result<Client, reqwest::Error> {
//}

//trait Cookies {
//    fn store_cookies(&str) -> Result<(),()>;
//}

