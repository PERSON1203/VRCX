//! # vrchat-api
//!
//! An async Rust client library for the [VRChat API](https://api.vrchat.cloud/api/1).
//!
//! ## Quick start
//!
//! ```rust,no_run
//! use vrchat_api::{VRChatClient, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let client = VRChatClient::new()?;
//!
//!     // 1. Optionally fetch config (no auth required)
//!     let config = client.get_config().await?;
//!     println!("config: {}", config["clientApiKey"]);
//!
//!     // 2. Login
//!     let me = client.login("username", "password").await?;
//!     // If VRChat returns a 2FA challenge, call one of:
//!     //   client.verify_totp("123456").await?;
//!     //   client.verify_otp("123456").await?;
//!     //   client.verify_email_otp("123456").await?;
//!
//!     println!("Logged in as: {}", me.display_name);
//!
//!     // 3. Fetch friends
//!     use vrchat_api::api::friend::GetFriendsParams;
//!     let friends = client
//!         .get_friends(&GetFriendsParams { n: Some(50), ..Default::default() })
//!         .await?;
//!     println!("Online friends: {}", friends.len());
//!
//!     Ok(())
//! }
//! ```

pub mod api;
pub mod client;
pub mod error;
pub mod models;

pub use client::VRChatClient;
pub use error::{Result, VRChatError};
