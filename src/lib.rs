extern crate chrono;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

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
    workspaces: Option<Vec<crate::workspace::Workspace>>,
    projects: Option<Vec<crate::project::Project>>,
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
