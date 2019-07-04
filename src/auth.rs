static AUTH_URL: &'static str = "https://www.toggl.com/api/v8/me";

pub struct Toggl {
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
    let mut headers = reqwest::header::HeaderMap::default();
    let headerval: reqwest::header::HeaderValue = format!("Basic {}:{}", api_token, "api_token").parse()?;
    headers.insert(reqwest::header::AUTHORIZATION, headerval);

    let client = reqwest::Client::builder()
        .gzip(true)
        .default_headers(headers)
        .build()?;
    let resp = client.get(AUTH_URL)
        .send()?;
    if resp.status().is_success() {
        Ok(Toggl {
            client,
        })
    } else {
        Err(crate::error::TogglError::AuthError("authentication not succeded".to_owned()))
    }
}


//fn load_cookie(api_token) -> Result<Client, reqwest::Error> {
//}

//trait Cookies {
//    fn store_cookies(&str) -> Result<(),()>;
//}

