//! This is a library to interact with the toggl.com api version 8..
//! Your main interaction with the api will be the Toggl struct. Methods are in the TogglExt trait including the main objects being `Project` and `TimeEntry`.
//!
//! # Example
//! ```no_run
//! use toggl_rs::{TimeEntry, Toggl, TogglExt};
//! use toggl_rs::time_entry::TimeEntryExt;
//!
//! const API_TOKEN: &str = "token";
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let toggl = Toggl::init(API_TOKEN)?;
//!     let project = toggl.projects[0].clone();
//!
//!     let _ = toggl.start_entry(Some(String::from("test")), &[], Some(project))?;
//!
//!     let current_entry = toggl.get_running_entry()?;
//!     println!("{:?}", current_entry);
//!     toggl.stop_entry(&current_entry.unwrap())?;
//!
//!     Ok(())
//! }
//! ```
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

pub use crate::error::TogglError;
use crate::project::ProjectTrait;
pub use crate::time_entry::TimeEntryExt as TogglExt;
pub use crate::types::TimeEntry;

/// Call this to get a toggl object on which you can call various methods.
/// This will be hour handler to the api.
/// Notice, that this will already query the api.
impl Toggl {
    pub fn init(api_token: &str) -> Result<Toggl, crate::error::TogglError> {
        let mut t = Toggl::authenticate_api_token(api_token)?;
        t.fill_projects();
        Ok(t)
    }
}

#[derive(Debug)]
/// The main struct to interact with.
pub struct Toggl {
    api_token: String,
    client: reqwest::blocking::Client,
    /// Information of the user.
    pub user: crate::user::User,
    /// A handler to all projects currently available in Toggl.
    pub projects: Vec<Rc<crate::project::Project>>,
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
        let resp = self
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
            .and_then(|v| v.json::<S>())
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
            .and_then(|v| v.json())
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
            .and_then(|v| v.json())
            .map_err(|e| e.into())
    }
}
