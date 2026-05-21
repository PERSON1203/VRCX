use serde::{Deserialize, Serialize};
use serde_json::Value;


// ---------------------------------------------------------------------------
// User (public profile returned by GET /users/{userId})
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub display_name: String,
    pub bio: String,
    pub bio_links: Vec<String>,
    pub current_avatar_image_url: String,
    pub current_avatar_thumbnail_image_url: String,
    pub current_avatar_tags: Vec<String>,
    pub developer_type: String,
    pub is_friend: bool,
    pub last_activity: String,
    pub last_login: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_mobile: Option<String>,
    pub last_platform: String,
    pub profile_pic_override: String,
    pub profile_pic_override_thumbnail: String,
    pub pronouns: String,
    pub state: String,
    pub status: String,
    pub status_description: String,
    pub tags: Vec<String>,
    pub user_icon: String,
    pub age_verification_status: String,
    pub age_verified: bool,
    pub allow_avatar_copying: bool,
    pub date_joined: String,
    pub discord_id: String,
    pub friend_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friend_request_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traveling_to_instance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traveling_to_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traveling_to_world: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world_id: Option<String>,
    pub badges: Vec<Badge>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

// ---------------------------------------------------------------------------
// CurrentUser – the authenticated user; superset of User
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CurrentUser {
    // --- inherited User fields ---
    pub id: String,
    pub display_name: String,
    pub bio: String,
    pub bio_links: Vec<String>,
    pub current_avatar_image_url: String,
    pub current_avatar_thumbnail_image_url: String,
    pub current_avatar_tags: Vec<String>,
    pub developer_type: String,
    pub is_friend: bool,
    pub last_activity: String,
    pub last_login: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_mobile: Option<String>,
    pub last_platform: String,
    pub profile_pic_override: String,
    pub profile_pic_override_thumbnail: String,
    pub pronouns: String,
    pub state: String,
    pub status: String,
    pub status_description: String,
    pub tags: Vec<String>,
    pub user_icon: String,
    pub age_verification_status: String,
    pub age_verified: bool,
    pub allow_avatar_copying: bool,
    pub date_joined: String,
    pub discord_id: String,
    pub friend_key: String,
    pub badges: Vec<Badge>,
    // --- CurrentUser-only fields ---
    pub accepted_privacy_version: u32,
    pub accepted_tos_version: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_deletion_date: Option<String>,
    pub active_friends: Vec<String>,
    pub current_avatar: String,
    pub email_verified: bool,
    pub fallback_avatar: String,
    pub friend_group_names: Vec<String>,
    pub friends: Vec<String>,
    pub has_birthday: bool,
    pub has_email: bool,
    pub has_logged_in_from_client: bool,
    pub has_pending_email: bool,
    pub home_location: String,
    pub is_adult: bool,
    pub is_booping_enabled: bool,
    pub obfuscated_email: String,
    pub obfuscated_pending_email: String,
    pub oculus_id: String,
    pub offline_friends: Vec<String>,
    pub online_friends: Vec<String>,
    pub past_display_names: Vec<PastDisplayName>,
    pub pico_id: String,
    pub presence: Option<UserPresence>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Badge {
    pub badge_id: String,
    pub badge_name: String,
    pub badge_description: String,
    pub badge_image_url: String,
    pub showcased: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PastDisplayName {
    pub display_name: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserPresence {
    pub id: String,
    pub display_name: String,
    pub status: String,
    pub world: String,
    pub instance: String,
    pub instance_type: String,
    pub platform: String,
    pub avatar_thumbnail: String,
    pub profile_pic_override: String,
    pub user_icon: String,
    pub current_avatar_tags: String,
    pub debug_flag: String,
    pub groups: Vec<String>,
    pub traveling_to_world: String,
    pub traveling_to_instance: String,
}

// ---------------------------------------------------------------------------
// UserNote
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserNote {
    pub id: String,
    pub user_id: String,
    pub target_user_id: String,
    pub note: String,
    pub created_at: String,
    pub updated_at: String,
}

// ---------------------------------------------------------------------------
// FriendStatus
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FriendStatus {
    pub is_friend: bool,
    pub outgoing_request: bool,
    pub incoming_request: bool,
}

// ---------------------------------------------------------------------------
// MutualCounts
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MutualCounts {
    pub mutual_friend_count: u32,
    pub mutual_group_count: u32,
}
