/// Error Value
#[derive(Debug)]
pub enum TogglError {
    /// All errors that come from authentication.
    AuthError(String),
    /// Errors that come from reqwest throwing an error
    ReqwestError(reqwest::Error),
    /// Dummy Type. Not used in the API
    NotImplemented,
}

impl std::convert::From<reqwest::Error> for crate::error::TogglError {
    fn from(e: reqwest::Error) -> crate::error::TogglError {
        crate::error::TogglError::ReqwestError(e)
    }
}

impl std::convert::From<reqwest::header::InvalidHeaderValue> for crate::error::TogglError {
    fn from(_e: reqwest::header::InvalidHeaderValue) -> crate::error::TogglError {
        crate::error::TogglError::AuthError("Could not parse Authentication api_token".to_owned())
    }
}