use serde::Serialize;

use crate::client::VRChatClient;
use crate::error::Result;
use crate::models::World;

// ---------------------------------------------------------------------------
// Params
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetWorldsParams {
    pub n: Option<u32>,
    pub offset: Option<u32>,
    pub search: Option<String>,
    pub user_id: Option<String>,
    pub user: Option<String>,
    pub sort: Option<String>,
    pub order: Option<String>,
    pub release_status: Option<String>,
    pub featured: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SaveWorldParams {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub capacity: Option<u32>,
    pub recommended_capacity: Option<u32>,
    pub preview_youtube_id: Option<String>,
    pub url_list: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
}

// ---------------------------------------------------------------------------
// API impl
// ---------------------------------------------------------------------------

impl VRChatClient {
    /// GET /worlds/{worldId} – Get a world by ID.
    pub async fn get_world(&self, world_id: &str) -> Result<World> {
        self.get(&format!("worlds/{world_id}"), None::<&()>).await
    }

    /// GET /worlds – Search worlds.  Pass an optional `option` (e.g. `"active"`,
    /// `"recent"`, `"favorites"`) to hit `/worlds/{option}`.
    pub async fn get_worlds(
        &self,
        params: &GetWorldsParams,
        option: Option<&str>,
    ) -> Result<Vec<World>> {
        let endpoint = match option {
            Some(opt) => format!("worlds/{opt}"),
            None => "worlds".to_owned(),
        };
        self.get(&endpoint, Some(params)).await
    }

    /// PUT /worlds/{worldId} – Update a world's metadata.
    pub async fn save_world(&self, params: &SaveWorldParams) -> Result<World> {
        self.put(&format!("worlds/{}", params.id), Some(params)).await
    }

    /// DELETE /worlds/{worldId} – Delete a world.
    pub async fn delete_world(&self, world_id: &str) -> Result<serde_json::Value> {
        self.delete(&format!("worlds/{world_id}"), None::<&()>)
            .await
    }

    /// PUT /worlds/{worldId}/publish – Publish a world.
    pub async fn publish_world(&self, world_id: &str) -> Result<World> {
        self.put::<_, ()>(&format!("worlds/{world_id}/publish"), None)
            .await
    }

    /// DELETE /worlds/{worldId}/publish – Unpublish a world.
    pub async fn unpublish_world(&self, world_id: &str) -> Result<World> {
        self.delete(&format!("worlds/{world_id}/publish"), None::<&()>)
            .await
    }

    /// GET /worlds/favorites – List favourite worlds.
    pub async fn get_favorite_worlds(&self, params: &GetWorldsParams) -> Result<Vec<World>> {
        self.get("worlds/favorites", Some(params)).await
    }
}
