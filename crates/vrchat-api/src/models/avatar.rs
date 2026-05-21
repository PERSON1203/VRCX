use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::common::UnityPackage;

// ---------------------------------------------------------------------------
// Avatar
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Avatar {
    pub id: String,
    pub name: String,
    pub author_id: String,
    pub author_name: String,
    pub description: String,
    pub image_url: String,
    pub thumbnail_image_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub release_status: String,
    pub tags: Vec<String>,
    pub featured: bool,
    pub unity_packages: Vec<UnityPackage>,
    pub version: u32,
    pub unity_package_url: String,
    pub searchable: bool,
    pub pending_upload: bool,
    pub acknowledgements: Option<String>,
    #[serde(default)]
    pub performance: std::collections::HashMap<String, Value>,
    #[serde(default)]
    pub styles: AvatarStyles,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AvatarStyles {
    pub primary: Option<String>,
    pub secondary: Option<String>,
}

// ---------------------------------------------------------------------------
// AvatarStyle (from GET /avatarStyles)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AvatarStyle {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

// ---------------------------------------------------------------------------
// AvatarModeration
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AvatarModeration {
    pub id: String,
    pub source_user_id: String,
    pub target_avatar_id: String,
    pub avatar_moderation_type: String,
    pub created: String,
}
