extern crate chrono;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

use std::rc::Rc;

mod auth;
mod error;
pub mod project;
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
    fn post<T: serde::ser::Serialize>(
        &self,
        url: &str,
        t: &T,
    ) -> Result<(), crate::error::TogglError>;
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

    fn post<T: serde::ser::Serialize>(
        &self,
        url: &str,
        t: &T,
    ) -> Result<(), crate::error::TogglError> {

        panic!("I need to parse the response to json to test if there was an");
        self.client
            .post(url)
            .json(t)
            .basic_auth(&self.api_token, Some("api_token"))
            .send()
            .map(|_v| ())
            .map_err(|v| v.into())
    }

    fn put<T: serde::ser::Serialize>(
        &self,
        url: &str,
        t: &Option<T>,
    ) -> Result<(), crate::error::TogglError> {

        panic!("I need to parse the response to json to test if there was an");
        if let Some(l) = t {
            self.client
                .put(url)
                .json(l)
                .basic_auth(&self.api_token, Some("api_token"))
                .send()?;
        } else {
            self.client
                .put(url)
                .basic_auth(&self.api_token, Some("api_token"))
                .send()?;
        }
        Ok(())
    }

    fn delete(&self, url: &str) -> Result<(), crate::error::TogglError> {
        panic!("I need to parse the response to json to test if there was an");
        self.client
            .delete(url)
            .basic_auth(&self.api_token, Some("api_token"))
            .send()?;
        Ok(())
    }

}
