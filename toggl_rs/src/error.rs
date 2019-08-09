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
