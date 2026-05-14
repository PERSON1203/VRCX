use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::common::UnityPackage;

// ---------------------------------------------------------------------------
// World
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct World {
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
    pub capacity: u32,
    pub recommended_capacity: u32,
    pub favorites: u32,
    pub heat: u32,
    pub popularity: u32,
    pub publication_date: String,
    pub labs_publication_date: String,
    pub organization: String,
    pub udon_products: Vec<Value>,
    pub occupants: Option<u32>,
    pub private_occupants: Option<u32>,
    pub public_occupants: Option<u32>,
    pub version: Option<u32>,
    pub visits: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_youtube_id: Option<String>,
    #[serde(default)]
    pub url_list: Vec<String>,
    #[serde(default)]
    pub instances: Vec<Value>,
    #[serde(default)]
    pub default_content_settings: std::collections::HashMap<String, Value>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}
