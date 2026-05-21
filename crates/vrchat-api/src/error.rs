use thiserror::Error;

#[derive(Debug, Error)]
pub enum VRChatError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error {status_code}: {message}")]
    Api { status_code: u16, message: String },

    #[error("Authentication required")]
    Unauthenticated,

    #[error("Two-factor authentication required")]
    TwoFactorRequired,

    #[error("Rate limited (429)")]
    RateLimited,

    #[error("Not found (404): {0}")]
    NotFound(String),

    #[error("JSON deserialize error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("URL parse error: {0}")]
    Url(#[from] url::ParseError),

    #[error("{0}")]
    Other(String),
}

/// Convenience Result alias
pub type Result<T> = std::result::Result<T, VRChatError>;
