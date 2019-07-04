extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod auth;
mod error;
mod project;
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
