extern crate chrono;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

use reqwest::IntoUrl;
use std::rc::Rc;

mod auth;
mod error;
pub mod project;
pub mod time_entry;
mod types;
mod user;
mod workspace;

pub use crate::time_entry::TimeEntryExt as TogglExt;
pub use crate::types::TimeEntry;

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
    fn get<U: IntoUrl, T: serde::de::DeserializeOwned>(
        &self,
        url: U,
    ) -> Result<T, crate::error::TogglError>;
    fn post<U: IntoUrl, T: serde::ser::Serialize, S: serde::de::DeserializeOwned>(
        &self,
        url: U,
        t: &T,
    ) -> Result<S, crate::error::TogglError>;
    fn put<U: IntoUrl, T: serde::ser::Serialize, S: serde::de::DeserializeOwned>(
        &self,
        url: U,
        t: &T,
    ) -> Result<S, crate::error::TogglError>;
    fn delete<U: IntoUrl, S: serde::de::DeserializeOwned>(
        &self,
        url: U,
    ) -> Result<S, crate::error::TogglError>;
}

impl Query for Toggl {
    fn get<U: IntoUrl, T: serde::de::DeserializeOwned>(
        &self,
        url: U,
    ) -> Result<T, crate::error::TogglError> {
        let mut resp = self
            .client
            .get(url)
            .basic_auth(&self.api_token, Some("api_token"))
            .send()?;
        Ok(resp.json()?)
    }

    fn post<U: IntoUrl, T: serde::ser::Serialize, S: serde::de::DeserializeOwned>(
        &self,
        url: U,
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

    fn put<U: IntoUrl, T: serde::ser::Serialize, S: serde::de::DeserializeOwned>(
        &self,
        url: U,
        t: &T,
    ) -> Result<S, crate::error::TogglError> {
        self.client
            .put(url)
            .json(t)
            .basic_auth(&self.api_token, Some("api_token"))
            .send()
            .and_then(|mut v| v.json())
            .map_err(|e| e.into())
    }

    fn delete<U: IntoUrl, S: serde::de::DeserializeOwned>(
        &self,
        url: U,
    ) -> Result<S, crate::error::TogglError> {
        self.client
            .delete(url)
            .basic_auth(&self.api_token, Some("api_token"))
            .send()
            .and_then(|mut v| v.json())
            .map_err(|e| e.into())
    }
}
