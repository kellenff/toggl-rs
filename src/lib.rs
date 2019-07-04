extern crate chrono;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

use std::rc::Rc;

mod auth;
mod error;
mod project;
mod time_entry;
mod user;
mod workspace;

#[derive(Debug)]
pub struct Toggl {
    api_token: String,
    client: reqwest::Client,
    user: crate::user::User,
    projects: Option<Vec<Rc<crate::project::Project>>>,
}

trait Query<T: serde::de::DeserializeOwned> {
    fn query(&self, url: &str) -> Result<T, crate::error::TogglError>;
}

impl<T: serde::de::DeserializeOwned> Query<T> for Toggl {
    fn query(&self, url: &str) -> Result<T, crate::error::TogglError> {
        let mut resp = self
            .client
            .get(url)
            .basic_auth(&self.api_token, Some("api_token"))
            .send()?;
    Ok(resp
        .json()?)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_auth() {
        use crate::auth::init;
        let init = init("INVALID");
        assert!(init.is_ok())
    }


}
