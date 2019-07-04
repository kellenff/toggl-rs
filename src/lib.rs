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

#[cfg(test)]
mod tests {
    #[test]
    fn test_auth() {
        use crate::auth::init;
        let init = init("INVALID");
        assert!(init.is_ok())
    }


}
