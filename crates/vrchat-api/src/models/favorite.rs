use serde::{Deserialize, Serialize};
use serde_json::Value;

// ---------------------------------------------------------------------------
// Favorite
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Favorite {
    pub id: String,
    pub favorite_id: String,
    pub r#type: String,
    pub tags: Vec<String>,
}

// ---------------------------------------------------------------------------
// FavoriteGroup
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FavoriteGroup {
    pub id: String,
    pub owner_id: String,
    pub owner_display_name: String,
    pub name: String,
    pub display_name: String,
    pub r#type: String,
    pub visibility: String,
    pub tags: Vec<String>,
}

// ---------------------------------------------------------------------------
// FavoriteLimits
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FavoriteLimits {
    #[serde(flatten)]
    pub limits: std::collections::HashMap<String, Value>,
}
