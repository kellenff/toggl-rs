static COOKIE_AUTH_URL: &'static str = "https://www.toggl.com/api/v8/sessions";

pub struct Toggl<'a> {
    client: reqwest::Client,
    cookie: reqwest::cookie::Cookie<'a>,
}

pub enum AuthError {
    CLIENT_ERROR,
    COOKIE_ERROR,
}

impl std::convert::From<reqwest::Error> for AuthError {
    fn from(e: reqwest::Error) -> AuthError {
        CLIENT_ERROR
    }
}

impl std::convert::From<std::option::NoneError> for AuthError {
    fn from(e: std::option::NoneError) -> AuthError {
        COOKIE_ERROR
    }
}


pub fn init(api_token: &str) -> Result<Toggl, Error> {
    let client = reqwest::Client::new();
    let resp = client.post(COOKIE_AUTH_URL)
        .basic_auth(api_token, Some("api_token"))
        .send()?;
    let cookie = resp.cookies().next()?;

    Ok(Toggl {
        client,
        cookie
    })
}


//fn load_cookie(api_token) -> Result<Client, reqwest::Error> {
//}

//trait Cookies {
//    fn store_cookies(&str) -> Result<(),()>;
//}

