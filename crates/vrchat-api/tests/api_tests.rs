//! Integration tests for `vrchat-api` using a local mock HTTP server.
//!
//! Each test spins up a `mockito` server, configures `VRChatClient` to point
//! at it, and verifies that the correct HTTP request is sent and the response
//! is parsed into the expected type (or error variant).

use vrchat_api::{VRChatClient, VRChatError};
use vrchat_api::api::friend::GetFriendsParams;
use vrchat_api::api::user::GetUsersParams;
use vrchat_api::api::world::GetWorldsParams;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Create a `VRChatClient` pointing at the mock server root.
fn make_client(server_url: &str) -> VRChatClient {
    VRChatClient::with_base_url(server_url).expect("failed to build client")
}

// ---------------------------------------------------------------------------
// auth – GET /config
// ---------------------------------------------------------------------------

#[tokio::test]
async fn get_config_returns_value() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock("GET", "/config")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"clientApiKey": "test_key", "apiVersion": 1}"#)
        .create_async()
        .await;

    let client = make_client(&server.url());
    let config = client.get_config().await.unwrap();
    assert_eq!(config["clientApiKey"], "test_key");
    assert_eq!(config["apiVersion"], 1);
}

// ---------------------------------------------------------------------------
// auth – GET /auth/user (login)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn login_returns_current_user() {
    let mut server = mockito::Server::new_async().await;
    let body = current_user_json("usr_me", "TestUser");
    let _mock = server
        .mock("GET", "/auth/user")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(&body)
        .create_async()
        .await;

    let client = make_client(&server.url());
    let user = client.login("user", "pass").await.unwrap();
    assert_eq!(user.id, "usr_me");
    assert_eq!(user.display_name, "TestUser");
}

#[tokio::test]
async fn login_401_missing_credentials_returns_unauthenticated() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock("GET", "/auth/user")
        .with_status(401)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error":{"message":"Missing Credentials","status_code":401}}"#)
        .create_async()
        .await;

    let client = make_client(&server.url());
    let err = client.login("bad_user", "bad_pass").await.unwrap_err();
    assert!(
        matches!(err, VRChatError::Unauthenticated),
        "expected Unauthenticated, got: {err:?}"
    );
}

#[tokio::test]
async fn login_401_two_factor_required() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock("GET", "/auth/user")
        .with_status(401)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error":{"message":"Requires Two-Factor Auth","status_code":401}}"#)
        .create_async()
        .await;

    let client = make_client(&server.url());
    let err = client.login("user", "pass").await.unwrap_err();
    assert!(
        matches!(err, VRChatError::TwoFactorRequired),
        "expected TwoFactorRequired, got: {err:?}"
    );
}

// ---------------------------------------------------------------------------
// auth – GET /auth/user (get_current_user)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn get_current_user_success() {
    let mut server = mockito::Server::new_async().await;
    let body = current_user_json("usr_abc", "AnotherUser");
    let _mock = server
        .mock("GET", "/auth/user")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(&body)
        .create_async()
        .await;

    let client = make_client(&server.url());
    let user = client.get_current_user().await.unwrap();
    assert_eq!(user.id, "usr_abc");
}

// ---------------------------------------------------------------------------
// auth – 2FA verify endpoints
// ---------------------------------------------------------------------------

#[tokio::test]
async fn verify_totp_success() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock("POST", "/auth/twofactorauth/totp/verify")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"verified": true}"#)
        .create_async()
        .await;

    let client = make_client(&server.url());
    let resp = client.verify_totp("123456").await.unwrap();
    assert!(resp.verified);
}

#[tokio::test]
async fn verify_otp_success() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock("POST", "/auth/twofactorauth/otp/verify")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"verified": true, "token": "tok_abc"}"#)
        .create_async()
        .await;

    let client = make_client(&server.url());
    let resp = client.verify_otp("123456").await.unwrap();
    assert!(resp.verified);
    assert_eq!(resp.token.as_deref(), Some("tok_abc"));
}

#[tokio::test]
async fn verify_email_otp_success() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock("POST", "/auth/twofactorauth/emailotp/verify")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"verified": true}"#)
        .create_async()
        .await;

    let client = make_client(&server.url());
    let resp = client.verify_email_otp("654321").await.unwrap();
    assert!(resp.verified);
}

// ---------------------------------------------------------------------------
// friend – GET /auth/user/friends
// ---------------------------------------------------------------------------

#[tokio::test]
async fn get_friends_returns_list() {
    let mut server = mockito::Server::new_async().await;
    let body = format!(
        "[{}, {}]",
        user_json("usr_friend1", "FriendOne"),
        user_json("usr_friend2", "FriendTwo")
    );
    let _mock = server
        .mock("GET", mockito::Matcher::Regex(r"^/auth/user/friends(\?.*)?$".to_owned()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(&body)
        .create_async()
        .await;

    let client = make_client(&server.url());
    let friends = client
        .get_friends(&GetFriendsParams { n: Some(10), ..Default::default() })
        .await
        .unwrap();
    assert_eq!(friends.len(), 2);
    assert_eq!(friends[0].id, "usr_friend1");
    assert_eq!(friends[1].display_name, "FriendTwo");
}

#[tokio::test]
async fn get_friends_empty_list() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock("GET", mockito::Matcher::Regex(r"^/auth/user/friends(\?.*)?$".to_owned()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("[]")
        .create_async()
        .await;

    let client = make_client(&server.url());
    let friends = client
        .get_friends(&GetFriendsParams::default())
        .await
        .unwrap();
    assert!(friends.is_empty());
}

// ---------------------------------------------------------------------------
// friend – GET /user/{userId}/friendStatus
// ---------------------------------------------------------------------------

#[tokio::test]
async fn get_friend_status_is_friend() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock("GET", "/user/usr_other/friendStatus")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"isFriend":true,"outgoingRequest":false,"incomingRequest":false}"#)
        .create_async()
        .await;

    let client = make_client(&server.url());
    let status = client.get_friend_status("usr_other").await.unwrap();
    assert!(status.is_friend);
    assert!(!status.outgoing_request);
}

#[tokio::test]
async fn get_friend_status_pending_outgoing() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock("GET", "/user/usr_stranger/friendStatus")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"isFriend":false,"outgoingRequest":true,"incomingRequest":false}"#)
        .create_async()
        .await;

    let client = make_client(&server.url());
    let status = client.get_friend_status("usr_stranger").await.unwrap();
    assert!(!status.is_friend);
    assert!(status.outgoing_request);
}

// ---------------------------------------------------------------------------
// user – GET /users/{userId}
// ---------------------------------------------------------------------------

#[tokio::test]
async fn get_user_success() {
    let mut server = mockito::Server::new_async().await;
    let body = user_json("usr_xyz", "XYZUser");
    let _mock = server
        .mock("GET", "/users/usr_xyz")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(&body)
        .create_async()
        .await;

    let client = make_client(&server.url());
    let user = client.get_user("usr_xyz").await.unwrap();
    assert_eq!(user.id, "usr_xyz");
    assert_eq!(user.display_name, "XYZUser");
}

#[tokio::test]
async fn get_user_404_returns_not_found() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock("GET", "/users/usr_missing")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error":{"message":"User not found","status_code":404}}"#)
        .create_async()
        .await;

    let client = make_client(&server.url());
    let err = client.get_user("usr_missing").await.unwrap_err();
    assert!(
        matches!(err, VRChatError::NotFound(_)),
        "expected NotFound, got: {err:?}"
    );
    if let VRChatError::NotFound(msg) = err {
        assert!(msg.contains("User not found"));
    }
}

// ---------------------------------------------------------------------------
// user – GET /users
// ---------------------------------------------------------------------------

#[tokio::test]
async fn get_users_search() {
    let mut server = mockito::Server::new_async().await;
    let body = format!("[{}]", user_json("usr_result", "ResultUser"));
    let _mock = server
        .mock("GET", mockito::Matcher::Regex(r"^/users(\?.*)?$".to_owned()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(&body)
        .create_async()
        .await;

    let client = make_client(&server.url());
    let users = client
        .get_users(&GetUsersParams {
            search: Some("Result".to_owned()),
            n: Some(5),
            ..Default::default()
        })
        .await
        .unwrap();
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].id, "usr_result");
}

// ---------------------------------------------------------------------------
// world – GET /worlds/{worldId}
// ---------------------------------------------------------------------------

#[tokio::test]
async fn get_world_success() {
    let mut server = mockito::Server::new_async().await;
    let body = world_json("wrld_abc", "Test World");
    let _mock = server
        .mock("GET", "/worlds/wrld_abc")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(&body)
        .create_async()
        .await;

    let client = make_client(&server.url());
    let world = client.get_world("wrld_abc").await.unwrap();
    assert_eq!(world.id, "wrld_abc");
    assert_eq!(world.name, "Test World");
}

#[tokio::test]
async fn get_world_404_returns_not_found() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock("GET", "/worlds/wrld_gone")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error":{"message":"World not found","status_code":404}}"#)
        .create_async()
        .await;

    let client = make_client(&server.url());
    let err = client.get_world("wrld_gone").await.unwrap_err();
    assert!(matches!(err, VRChatError::NotFound(_)));
}

// ---------------------------------------------------------------------------
// world – GET /worlds
// ---------------------------------------------------------------------------

#[tokio::test]
async fn get_worlds_search() {
    let mut server = mockito::Server::new_async().await;
    let body = format!("[{}]", world_json("wrld_w1", "World One"));
    let _mock = server
        .mock("GET", mockito::Matcher::Regex(r"^/worlds(\?.*)?$".to_owned()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(&body)
        .create_async()
        .await;

    let client = make_client(&server.url());
    let worlds = client
        .get_worlds(&GetWorldsParams::default(), None)
        .await
        .unwrap();
    assert_eq!(worlds.len(), 1);
    assert_eq!(worlds[0].name, "World One");
}

#[tokio::test]
async fn get_worlds_with_option() {
    let mut server = mockito::Server::new_async().await;
    let body = format!("[{}]", world_json("wrld_active", "Active World"));
    let _mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(r"^/worlds/active(\?.*)?$".to_owned()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(&body)
        .create_async()
        .await;

    let client = make_client(&server.url());
    let worlds = client
        .get_worlds(&GetWorldsParams::default(), Some("active"))
        .await
        .unwrap();
    assert_eq!(worlds[0].id, "wrld_active");
}

// ---------------------------------------------------------------------------
// Error mapping – 429 rate limited
// ---------------------------------------------------------------------------

#[tokio::test]
async fn rate_limited_error_mapping() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock("GET", "/config")
        .with_status(429)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error":{"message":"Too Many Requests","status_code":429}}"#)
        .create_async()
        .await;

    let client = make_client(&server.url());
    let err = client.get_config().await.unwrap_err();
    assert!(
        matches!(err, VRChatError::RateLimited),
        "expected RateLimited, got: {err:?}"
    );
}

// ---------------------------------------------------------------------------
// Error mapping – generic API error
// ---------------------------------------------------------------------------

#[tokio::test]
async fn generic_api_error_mapping() {
    let mut server = mockito::Server::new_async().await;
    let _mock = server
        .mock("GET", "/config")
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error":{"message":"Bad Request","status_code":400}}"#)
        .create_async()
        .await;

    let client = make_client(&server.url());
    let err = client.get_config().await.unwrap_err();
    match err {
        VRChatError::Api { status_code, message } => {
            assert_eq!(status_code, 400);
            assert_eq!(message, "Bad Request");
        }
        other => panic!("expected Api error, got: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// JSON fixture helpers
// ---------------------------------------------------------------------------

fn user_json(id: &str, display_name: &str) -> String {
    format!(
        r#"{{
            "id": "{id}",
            "displayName": "{display_name}",
            "bio": "",
            "bioLinks": [],
            "currentAvatarImageUrl": "",
            "currentAvatarThumbnailImageUrl": "",
            "currentAvatarTags": [],
            "developerType": "none",
            "isFriend": false,
            "lastActivity": "",
            "lastLogin": "",
            "lastPlatform": "standalonewindows",
            "profilePicOverride": "",
            "profilePicOverrideThumbnail": "",
            "pronouns": "",
            "state": "offline",
            "status": "active",
            "statusDescription": "",
            "tags": [],
            "userIcon": "",
            "ageVerificationStatus": "hidden",
            "ageVerified": false,
            "allowAvatarCopying": false,
            "dateJoined": "2021-01-01",
            "discordId": "",
            "friendKey": "",
            "badges": []
        }}"#
    )
}

fn current_user_json(id: &str, display_name: &str) -> String {
    format!(
        r#"{{
            "id": "{id}",
            "displayName": "{display_name}",
            "bio": "",
            "bioLinks": [],
            "currentAvatarImageUrl": "",
            "currentAvatarThumbnailImageUrl": "",
            "currentAvatarTags": [],
            "developerType": "none",
            "isFriend": false,
            "lastActivity": "",
            "lastLogin": "",
            "lastPlatform": "standalonewindows",
            "profilePicOverride": "",
            "profilePicOverrideThumbnail": "",
            "pronouns": "",
            "state": "online",
            "status": "active",
            "statusDescription": "",
            "tags": [],
            "userIcon": "",
            "ageVerificationStatus": "hidden",
            "ageVerified": false,
            "allowAvatarCopying": false,
            "dateJoined": "2021-01-01",
            "discordId": "",
            "friendKey": "",
            "badges": [],
            "acceptedPrivacyVersion": 1,
            "acceptedTosVersion": 8,
            "activeFriends": [],
            "currentAvatar": "avtr_default",
            "emailVerified": true,
            "fallbackAvatar": "avtr_fallback",
            "friendGroupNames": [],
            "friends": [],
            "hasBirthday": true,
            "hasEmail": true,
            "hasLoggedInFromClient": false,
            "hasPendingEmail": false,
            "homeLocation": "",
            "isAdult": true,
            "isBoopingEnabled": false,
            "obfuscatedEmail": "t***@example.com",
            "obfuscatedPendingEmail": "",
            "oculusId": "",
            "offlineFriends": [],
            "onlineFriends": [],
            "pastDisplayNames": [],
            "picoId": "",
            "presence": null
        }}"#
    )
}

fn world_json(id: &str, name: &str) -> String {
    format!(
        r#"{{
            "id": "{id}",
            "name": "{name}",
            "authorId": "usr_author",
            "authorName": "Author",
            "description": "",
            "imageUrl": "",
            "thumbnailImageUrl": "",
            "createdAt": "2021-01-01T00:00:00.000Z",
            "updatedAt": "2024-01-01T00:00:00.000Z",
            "releaseStatus": "public",
            "tags": [],
            "featured": false,
            "unityPackages": [],
            "capacity": 32,
            "recommendedCapacity": 16,
            "favorites": 0,
            "heat": 0,
            "popularity": 0,
            "publicationDate": "2021-01-01",
            "labsPublicationDate": "none",
            "organization": "vrchat",
            "udonProducts": []
        }}"#
    )
}
