use reqwest::{
    header::{HeaderMap, HeaderValue, USER_AGENT},
    Client, Method, StatusCode,
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use crate::error::{Result, VRChatError};

pub const BASE_URL: &str = "https://api.vrchat.cloud/api/1";
const DEFAULT_USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

/// Central VRChat API client.
///
/// Maintains a cookie jar so that the auth session is preserved across calls.
/// Construct with [`VRChatClient::new`] and then call [`VRChatClient::login`]
/// before making authenticated requests.
#[derive(Clone, Debug)]
pub struct VRChatClient {
    pub(crate) client: Client,
    pub(crate) base_url: String,
}

impl VRChatClient {
    /// Create a new client using the default VRChat API base URL.
    pub fn new() -> Result<Self> {
        Self::with_base_url(BASE_URL)
    }

    /// Create a new client using a custom base URL (useful for testing).
    pub fn with_base_url(base_url: impl Into<String>) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static(DEFAULT_USER_AGENT),
        );

        let client = Client::builder()
            .cookie_store(true)
            .default_headers(headers)
            .build()?;

        Ok(Self {
            client,
            base_url: base_url.into(),
        })
    }

    /// Build the full URL for a given endpoint path.
    pub(crate) fn url(&self, endpoint: &str) -> String {
        format!("{}/{}", self.base_url, endpoint)
    }

    /// Execute a GET request, serialising `params` as query string parameters.
    pub(crate) async fn get<T, P>(&self, endpoint: &str, params: Option<&P>) -> Result<T>
    where
        T: DeserializeOwned,
        P: Serialize + ?Sized,
    {
        let mut req = self.client.get(self.url(endpoint));
        if let Some(p) = params {
            req = req.query(p);
        }
        self.execute(req).await
    }

    /// Execute a POST request with a JSON body.
    pub(crate) async fn post<T, B>(&self, endpoint: &str, body: Option<&B>) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let mut req = self.client.post(self.url(endpoint));
        if let Some(b) = body {
            req = req.json(b);
        } else {
            req = req
                .header("Content-Type", "application/json;charset=utf-8")
                .body("{}");
        }
        self.execute(req).await
    }

    /// Execute a PUT request with a JSON body.
    pub(crate) async fn put<T, B>(&self, endpoint: &str, body: Option<&B>) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let mut req = self.client.put(self.url(endpoint));
        if let Some(b) = body {
            req = req.json(b);
        } else {
            req = req
                .header("Content-Type", "application/json;charset=utf-8")
                .body("{}");
        }
        self.execute(req).await
    }

    /// Execute a DELETE request, optionally serialising `body` as JSON.
    pub(crate) async fn delete<T, B>(&self, endpoint: &str, body: Option<&B>) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let mut req = self.client.request(Method::DELETE, self.url(endpoint));
        if let Some(b) = body {
            req = req.json(b);
        }
        self.execute(req).await
    }

    /// Execute a GET request using HTTP Basic Authentication.
    pub(crate) async fn get_basic_auth<T, P>(
        &self,
        endpoint: &str,
        username: &str,
        password: &str,
        params: Option<&P>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        P: Serialize + ?Sized,
    {
        let mut req = self
            .client
            .get(self.url(endpoint))
            .basic_auth(username, Some(password));
        if let Some(p) = params {
            req = req.query(p);
        }
        self.execute(req).await
    }

    /// Send the built request and map the response to `T`.
    async fn execute<T>(&self, request: reqwest::RequestBuilder) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = request.send().await?;
        let status = response.status();
        let body: Value = response.json().await?;

        // Detect API-level errors (VRChat wraps them in {"error": {...}})
        if let Some(err_obj) = body.get("error") {
            let message = err_obj
                .get("message")
                .and_then(Value::as_str)
                .unwrap_or("unknown error")
                .to_owned();
            let code = err_obj
                .get("status_code")
                .and_then(Value::as_u64)
                .unwrap_or(status.as_u16() as u64) as u16;

            if status == StatusCode::UNAUTHORIZED {
                if message.contains("Missing Credentials") {
                    return Err(VRChatError::Unauthenticated);
                }
                if message.contains("Requires Two-Factor") || message.contains("2FA") {
                    return Err(VRChatError::TwoFactorRequired);
                }
            }
            if status == StatusCode::TOO_MANY_REQUESTS {
                return Err(VRChatError::RateLimited);
            }
            if status == StatusCode::NOT_FOUND {
                return Err(VRChatError::NotFound(message));
            }
            return Err(VRChatError::Api {
                status_code: code,
                message,
            });
        }

        if !status.is_success() && status != StatusCode::OK {
            return Err(VRChatError::Api {
                status_code: status.as_u16(),
                message: body.to_string(),
            });
        }

        Ok(serde_json::from_value(body)?)
    }
}

impl Default for VRChatClient {
    fn default() -> Self {
        Self::new().expect("failed to build VRChatClient")
    }
}
