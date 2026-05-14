use serde::{Deserialize, Serialize};
use serde_json::Value;

// ---------------------------------------------------------------------------
// UnityPackage
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnityPackage {
    pub id: String,
    pub asset_version: u32,
    pub created_at: String,
    pub platform: String,
    pub unity_version: String,
    pub variant: String,
    pub asset_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_rating: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impositorizer_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unity_sort_number: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world_signature: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

// ---------------------------------------------------------------------------
// File / FileVersion – used by the file-upload APIs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FileRecord {
    pub id: String,
    pub name: String,
    pub owner_id: String,
    pub mime_type: String,
    pub extension: String,
    pub tags: Vec<String>,
    pub versions: Vec<FileVersion>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FileVersion {
    pub version: u32,
    pub status: String,
    pub created_at: String,
    pub file: Option<FileVersionPart>,
    pub delta: Option<FileVersionPart>,
    pub signature: Option<FileVersionPart>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FileVersionPart {
    pub category: String,
    pub file_name: String,
    pub size_in_bytes: Option<u64>,
    pub md5: Option<String>,
    pub url: Option<String>,
    pub status: String,
    pub upload_id: Option<String>,
}
