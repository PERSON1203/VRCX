use serde::{Deserialize, Serialize};
use serde_json::Value;

// ---------------------------------------------------------------------------
// Instance
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    pub id: String,
    pub world_id: String,
    pub instance_id: String,
    pub name: String,
    pub location: String,
    pub owner_id: Option<String>,
    pub r#type: String,
    pub region: String,
    pub capacity: u32,
    pub user_count: u32,
    pub n_users: u32,
    pub platforms: InstancePlatforms,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure_name: Option<String>,
    pub can_request_invite: bool,
    pub full: bool,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InstancePlatforms {
    pub standalonewindows: Option<u32>,
    pub android: Option<u32>,
}

// ---------------------------------------------------------------------------
// ShortName – returned by getInstanceShortName
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InstanceShortName {
    pub short_name: String,
    pub secure_name: Option<String>,
}
