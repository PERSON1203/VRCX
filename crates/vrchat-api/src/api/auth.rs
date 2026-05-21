use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::client::VRChatClient;
use crate::error::Result;

// ---------------------------------------------------------------------------
// Params
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct VerifyTwoFactorParams {
    pub code: String,
}

// ---------------------------------------------------------------------------
// Responses
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct TwoFactorVerifyResponse {
    pub verified: bool,
    #[serde(default)]
    pub token: Option<String>,
}

// ---------------------------------------------------------------------------
// API impl
// ---------------------------------------------------------------------------

impl VRChatClient {
    /// GET /config – Fetch server configuration (no auth required).
    pub async fn get_config(&self) -> Result<Value> {
        self.get("config", None::<&()>).await
    }

    /// Verify an OTP two-factor code (`POST /auth/twofactorauth/otp/verify`).
    pub async fn verify_otp(&self, code: impl Into<String>) -> Result<TwoFactorVerifyResponse> {
        let body = VerifyTwoFactorParams { code: code.into() };
        self.post("auth/twofactorauth/otp/verify", Some(&body)).await
    }

    /// Verify a TOTP two-factor code (`POST /auth/twofactorauth/totp/verify`).
    pub async fn verify_totp(&self, code: impl Into<String>) -> Result<TwoFactorVerifyResponse> {
        let body = VerifyTwoFactorParams { code: code.into() };
        self.post("auth/twofactorauth/totp/verify", Some(&body)).await
    }

    /// Verify an email OTP two-factor code
    /// (`POST /auth/twofactorauth/emailotp/verify`).
    pub async fn verify_email_otp(
        &self,
        code: impl Into<String>,
    ) -> Result<TwoFactorVerifyResponse> {
        let body = VerifyTwoFactorParams { code: code.into() };
        self.post("auth/twofactorauth/emailotp/verify", Some(&body))
            .await
    }

    /// Login with username/password using HTTP Basic Auth
    /// (`GET /auth/user`).  On success the session cookie is stored.
    pub async fn login(
        &self,
        username: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> Result<crate::models::CurrentUser> {
        self.get_basic_auth(
            "auth/user",
            username.as_ref(),
            password.as_ref(),
            None::<&()>,
        )
        .await
    }

    /// Refresh/check the current authenticated user (`GET /auth/user`).
    pub async fn get_current_user(&self) -> Result<crate::models::CurrentUser> {
        self.get("auth/user", None::<&()>).await
    }
}
