extern crate chrono;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

use std::rc::Rc;

mod auth;
mod error;
pub mod project;
mod return_types;
pub mod time_entry;
mod user;
mod workspace;

pub fn init(api_token: &str) -> Result<Toggl, crate::error::TogglError> {
    auth::init(api_token)
}

#[derive(Debug)]
pub struct Toggl {
    api_token: String,
    client: reqwest::Client,
    user: crate::user::User,
    pub projects: Option<Vec<Rc<crate::project::Project>>>,
}

trait Query {
    fn get<T: serde::de::DeserializeOwned>(&self, url: &str)
        -> Result<T, crate::error::TogglError>;
    fn post<T: serde::ser::Serialize, S: serde::de::DeserializeOwned>(
        &self,
        url: &str,
        t: &T,
    ) -> Result<S, crate::error::TogglError>;
    fn put<T: serde::ser::Serialize>(
        &self,
        url: &str,
        t: &Option<T>,
    ) -> Result<(), crate::error::TogglError>;
    fn delete(&self, url: &str) -> Result<(), crate::error::TogglError>;
}

impl Query for Toggl {
    fn get<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<T, crate::error::TogglError> {
        let mut resp = self
            .client
            .get(url)
            .basic_auth(&self.api_token, Some("api_token"))
            .send()?;
        Ok(resp.json()?)
    }

    fn post<T: serde::ser::Serialize, S: serde::de::DeserializeOwned>(
        &self,
        url: &str,
        t: &T,
    ) -> Result<S, crate::error::TogglError> {

        self.client
            .post(url)
            .json(t)
            .basic_auth(&self.api_token, Some("api_token"))
            .send()
            .and_then(|mut v| v.json::<S>())
            .map_err(|v| v.into())
    }

    fn put<T: serde::ser::Serialize>(
        &self,
        url: &str,
        t: &Option<T>,
    ) -> Result<(), crate::error::TogglError> {
        panic!("Fix error handling");
        if let Some(l) = t {
            self.client
                .put(url)
                .json(l)
                .basic_auth(&self.api_token, Some("api_token"))
                .send()
                .and_then(|mut v| v.json())?;
        } else {
            self.client
                .put(url)
                .basic_auth(&self.api_token, Some("api_token"))
                .send()
                .and_then(|mut v| v.json())?;
        }
        Ok(())
    }

    fn delete(&self, url: &str) -> Result<(), crate::error::TogglError> {
        panic!("Fix error handling");
        self.client
            .delete(url)
            .basic_auth(&self.api_token, Some("api_token"))
            .send()
            .and_then(|mut v| v.json())?;
        Ok(())
    }

}
