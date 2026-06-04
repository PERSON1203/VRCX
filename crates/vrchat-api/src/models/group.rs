use serde::{Deserialize, Serialize};
use serde_json::Value;

// ---------------------------------------------------------------------------
// Group
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: String,
    pub name: String,
    pub short_code: String,
    pub discriminator: String,
    pub description: String,
    pub icon_id: Option<String>,
    pub icon_url: Option<String>,
    pub banner_id: Option<String>,
    pub banner_url: Option<String>,
    pub owner_id: String,
    pub member_count: u32,
    pub member_count_synced_at: String,
    pub is_verified: bool,
    pub join_state: String,
    pub privacy: String,
    pub tags: Vec<String>,
    pub languages: Vec<String>,
    pub links: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub roles: Vec<GroupRole>,
    #[serde(default)]
    pub my_member: Option<GroupMember>,
    pub membership_status: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

// ---------------------------------------------------------------------------
// GroupMember
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GroupMember {
    pub id: String,
    pub group_id: String,
    pub user_id: String,
    pub joined_at: String,
    #[serde(default)]
    pub role_ids: Vec<String>,
    pub membership_status: String,
    pub is_subscribed_to_announcements: bool,
    pub visibility: String,
    pub is_representing: bool,
    pub has_joined_from_purchase: bool,
    pub is_banned: bool,
    pub ban_description: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

// ---------------------------------------------------------------------------
// GroupRole
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GroupRole {
    pub id: String,
    pub group_id: String,
    pub name: String,
    pub description: String,
    pub is_management_role: bool,
    pub permissions: Vec<String>,
    pub is_self_assignable: bool,
    pub requires_two_factor: bool,
    pub requires_purchase: bool,
    pub created_at: String,
    pub updated_at: String,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

// ---------------------------------------------------------------------------
// GroupPost
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GroupPost {
    pub id: String,
    pub group_id: String,
    pub author_id: String,
    pub title: String,
    pub text: String,
    pub image_id: Option<String>,
    pub image_url: Option<String>,
    pub visibility: String,
    pub role_ids_to_show: Vec<String>,
    pub likes: u32,
    pub created_at: String,
    pub updated_at: String,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

// ---------------------------------------------------------------------------
// GroupAuditLog
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GroupAuditLog {
    pub id: String,
    pub group_id: String,
    pub actor_id: String,
    pub actor_displayname: Option<String>,
    pub target_id: Option<String>,
    pub event_type: String,
    pub description: String,
    pub data: Option<Value>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GroupAuditLogList {
    pub results: Vec<GroupAuditLog>,
    pub total_count: u32,
    pub has_next: bool,
}

// ---------------------------------------------------------------------------
// GroupGallery / GroupGalleryImage
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GroupGalleryImage {
    pub id: String,
    pub group_id: String,
    pub file_id: String,
    pub image_url: String,
    pub created_by_user_id: String,
    pub created_at: String,
    pub approved: bool,
}

// ---------------------------------------------------------------------------
// CalendarEvent
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEvent {
    pub id: String,
    pub group_id: String,
    pub name: String,
    pub description: Option<String>,
    pub start_dt: String,
    pub end_dt: Option<String>,
    pub frequency: Option<String>,
    pub image_id: Option<String>,
    pub image_url: Option<String>,
    pub is_public: bool,
    pub visibility: String,
    pub is_following: Option<bool>,
    pub duration_in_ms: Option<u64>,
    pub occurrence_kind: Option<String>,
    pub recurrence: Option<String>,
    pub series_id: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}
