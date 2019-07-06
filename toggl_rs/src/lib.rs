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

trait Query {
    fn get<T: serde::de::DeserializeOwned>(&self, url: &str) -> Result<T, crate::error::TogglError>;
    fn post<T: serde::ser::Serialize>(&self, url: &str, t: &T) -> Result<(), crate::error::TogglError>;
    fn put<T: serde::ser::Serialize>(&self, url: &str) -> Result<(), crate::error::TogglError>;
    fn delete(&self, url: &str) -> Result<(), crate::error::TogglError>;
}

impl Query for Toggl {
    fn get<T: serde::de::DeserializeOwned>(&self, url: &str) -> Result<T, crate::error::TogglError> {
        let mut resp = self
            .client
            .get(url)
            .basic_auth(&self.api_token, Some("api_token"))
            .send()?;
    Ok(resp
        .json()?)
    }

    fn post<T: serde::ser::Serialize>(&self, url: &str, t: &T) -> Result<(), crate::error::TogglError> {
        self
            .client
            .post(url)
            .json(t)
            .basic_auth(&self.api_token, Some("api_token"))
            .send()?;
        Ok(())
    }

    fn put<T: serde::ser::Serialize>(&self, url: &str) -> Result<(), crate::error::TogglError> {
        self
            .client
            .put(url)
            .basic_auth(&self.api_token, Some("api_token"))
            .send()?;
        Ok(())
    }

    fn delete(&self, url: &str) -> Result<(), crate::error::TogglError> {
        self
            .client
            .delete(url)
            .basic_auth(&self.api_token, Some("api_token"))
            .send()?;
        Ok(())
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
