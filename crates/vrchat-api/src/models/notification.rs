use serde::{Deserialize, Serialize};
use serde_json::Value;

// ---------------------------------------------------------------------------
// Notification (v1 / v2)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: String,
    pub r#type: String,
    pub sender_user_id: String,
    pub sender_username: Option<String>,
    pub receiver_user_id: Option<String>,
    pub message: Option<String>,
    pub seen: bool,
    pub created_at: String,
    pub details: Option<Value>,
}

// ---------------------------------------------------------------------------
// InviteMessage
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InviteMessage {
    pub id: String,
    pub slot: u32,
    pub message: String,
    pub message_type: String,
    pub updated_at: String,
    pub remaining_cooldown_minutes: Option<u32>,
}

// ---------------------------------------------------------------------------
// PlayerModeration
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerModeration {
    pub id: String,
    pub r#type: String,
    pub source_user_id: String,
    pub source_display_name: String,
    pub target_user_id: String,
    pub target_display_name: String,
    pub created: String,
}
