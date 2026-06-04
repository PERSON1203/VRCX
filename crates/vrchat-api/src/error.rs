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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_error_display() {
        let e = VRChatError::Api {
            status_code: 403,
            message: "Forbidden".to_owned(),
        };
        assert_eq!(e.to_string(), "API error 403: Forbidden");
    }

    #[test]
    fn unauthenticated_display() {
        assert_eq!(
            VRChatError::Unauthenticated.to_string(),
            "Authentication required"
        );
    }

    #[test]
    fn two_factor_required_display() {
        assert_eq!(
            VRChatError::TwoFactorRequired.to_string(),
            "Two-factor authentication required"
        );
    }

    #[test]
    fn rate_limited_display() {
        assert_eq!(VRChatError::RateLimited.to_string(), "Rate limited (429)");
    }

    #[test]
    fn not_found_display() {
        let e = VRChatError::NotFound("world abc123".to_owned());
        assert_eq!(e.to_string(), "Not found (404): world abc123");
    }

    #[test]
    fn other_display() {
        let e = VRChatError::Other("something went wrong".to_owned());
        assert_eq!(e.to_string(), "something went wrong");
    }

    #[test]
    fn from_serde_json_error() {
        let raw = serde_json::from_str::<serde_json::Value>("not json");
        let err: VRChatError = raw.unwrap_err().into();
        assert!(err.to_string().contains("JSON deserialize error"));
    }

    #[test]
    fn from_url_parse_error() {
        let raw = "not a url ://".parse::<url::Url>();
        let err: VRChatError = raw.unwrap_err().into();
        assert!(err.to_string().contains("URL parse error"));
    }
}
