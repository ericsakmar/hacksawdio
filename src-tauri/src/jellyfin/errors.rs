use thiserror::Error;

#[derive(Debug, Error)]
pub enum JellyfinError {
    #[error("HTTP request failed: {0}")]
    HttpRequest(#[from] reqwest::Error),
    #[error("Failed to parse JSON response: {0}")]
    JsonParse(#[from] serde_json::Error),
    #[error("Jellyfin API error (status {status}): {message}")]
    ApiError {
        status: reqwest::StatusCode,
        message: String,
    },
    // #[error("Generic error: {0}")]
    // Generic(String),
}
