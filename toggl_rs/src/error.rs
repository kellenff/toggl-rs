#[derive(Debug)]
pub enum TogglError {
    AuthError(String),
    ReqwestError(reqwest::Error),
}