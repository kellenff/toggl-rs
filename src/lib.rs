pub mod auth;
pub mod error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_auth() {
        use crate::auth::init;
        assert!(init("INVALID").is_ok())
    }


}
