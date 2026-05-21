use serde::{Deserialize, Serialize};
use serde_json::Value;

// ---------------------------------------------------------------------------
// InventoryItem
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItem {
    pub id: String,
    pub owner_id: String,
    pub r#type: String,
    pub equip_slot: Option<String>,
    pub quantity: u32,
    pub is_archived: bool,
    pub created_at: String,
    pub updated_at: String,
    pub template_id: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

// ---------------------------------------------------------------------------
// InventoryTemplate
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InventoryTemplate {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub r#type: String,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

// ---------------------------------------------------------------------------
// VRChatBalance (credits)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VRChatBalance {
    pub balance: f64,
}

// ---------------------------------------------------------------------------
// Print
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Print {
    pub id: String,
    pub owner_id: String,
    pub image_url: String,
    pub created_at: String,
    pub note: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

// ---------------------------------------------------------------------------
// Prop
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Prop {
    pub id: String,
    pub name: String,
    pub author_id: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}
