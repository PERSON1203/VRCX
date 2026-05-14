use serde::Serialize;

use crate::client::VRChatClient;
use crate::error::Result;
use crate::models::{Avatar, AvatarStyle};

// ---------------------------------------------------------------------------
// Params
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAvatarsParams {
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
pub struct SaveAvatarParams {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub tags: Option<Vec<String>>,
    pub release_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLicensedAvatarsParams {
    pub n: Option<u32>,
    pub offset: Option<u32>,
}

// ---------------------------------------------------------------------------
// API impl
// ---------------------------------------------------------------------------

impl VRChatClient {
    /// GET /avatars/{avatarId} – Get a single avatar.
    pub async fn get_avatar(&self, avatar_id: &str) -> Result<Avatar> {
        self.get(&format!("avatars/{avatar_id}"), None::<&()>).await
    }

    /// GET /avatars – Search avatars.
    pub async fn get_avatars(&self, params: &GetAvatarsParams) -> Result<Vec<Avatar>> {
        self.get("avatars", Some(params)).await
    }

    /// PUT /avatars/{avatarId} – Update avatar metadata.
    pub async fn save_avatar(&self, params: &SaveAvatarParams) -> Result<Avatar> {
        self.put(&format!("avatars/{}", params.id), Some(params)).await
    }

    /// PUT /avatars/{avatarId}/select – Select this avatar for the current user.
    pub async fn select_avatar(&self, avatar_id: &str) -> Result<crate::models::CurrentUser> {
        self.put::<_, ()>(&format!("avatars/{avatar_id}/select"), None)
            .await
    }

    /// PUT /avatars/{avatarId}/selectfallback – Set fallback avatar.
    pub async fn select_fallback_avatar(
        &self,
        avatar_id: &str,
    ) -> Result<crate::models::CurrentUser> {
        self.put::<_, ()>(&format!("avatars/{avatar_id}/selectfallback"), None)
            .await
    }

    /// DELETE /avatars/{avatarId} – Delete an avatar.
    pub async fn delete_avatar(&self, avatar_id: &str) -> Result<serde_json::Value> {
        self.delete(&format!("avatars/{avatar_id}"), None::<&()>)
            .await
    }

    /// POST /avatars/{avatarId}/impostor/enqueue – Queue impostor generation.
    pub async fn create_imposter(&self, avatar_id: &str) -> Result<serde_json::Value> {
        self.post::<_, ()>(&format!("avatars/{avatar_id}/impostor/enqueue"), None)
            .await
    }

    /// DELETE /avatars/{avatarId}/impostor – Delete impostor.
    pub async fn delete_imposter(&self, avatar_id: &str) -> Result<serde_json::Value> {
        self.delete(&format!("avatars/{avatar_id}/impostor"), None::<&()>)
            .await
    }

    /// GET /avatarStyles – List available avatar styles.
    pub async fn get_available_avatar_styles(&self) -> Result<Vec<AvatarStyle>> {
        self.get("avatarStyles", None::<&()>).await
    }

    /// GET /avatars/licensed – List licensed avatars.
    pub async fn get_licensed_avatars(
        &self,
        params: &GetLicensedAvatarsParams,
    ) -> Result<Vec<Avatar>> {
        self.get("avatars/licensed", Some(params)).await
    }

    /// GET /avatars/favorites – List favourite avatars.
    pub async fn get_favorite_avatars(
        &self,
        params: &GetAvatarsParams,
    ) -> Result<Vec<Avatar>> {
        self.get("avatars/favorites", Some(params)).await
    }
}
