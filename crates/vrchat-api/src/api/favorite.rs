use serde::Serialize;

use crate::client::VRChatClient;
use crate::error::Result;
use crate::models::{Favorite, FavoriteGroup, FavoriteLimits};

// ---------------------------------------------------------------------------
// Params
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFavoritesParams {
    pub n: Option<u32>,
    pub offset: Option<u32>,
    pub r#type: Option<String>,
    pub tag: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddFavoriteParams {
    pub r#type: String,
    pub favorite_id: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFavoriteGroupsParams {
    pub n: Option<u32>,
    pub offset: Option<u32>,
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SaveFavoriteGroupParams {
    pub r#type: String,
    pub group: String,
    pub display_name: Option<String>,
    pub visibility: Option<String>,
}

// ---------------------------------------------------------------------------
// API impl
// ---------------------------------------------------------------------------

impl VRChatClient {
    /// GET /auth/user/favoritelimits – Get favorite limits.
    pub async fn get_favorite_limits(&self) -> Result<FavoriteLimits> {
        self.get("auth/user/favoritelimits", None::<&()>).await
    }

    /// GET /favorites – List favorites.
    pub async fn get_favorites(&self, params: &GetFavoritesParams) -> Result<Vec<Favorite>> {
        self.get("favorites", Some(params)).await
    }

    /// POST /favorites – Add a favorite.
    pub async fn add_favorite(&self, params: &AddFavoriteParams) -> Result<Favorite> {
        self.post("favorites", Some(params)).await
    }

    /// DELETE /favorites/{objectId} – Remove a favorite.
    pub async fn delete_favorite(&self, object_id: &str) -> Result<serde_json::Value> {
        self.delete(&format!("favorites/{object_id}"), None::<&()>)
            .await
    }

    /// GET /favorite/groups – List favorite groups.
    pub async fn get_favorite_groups(
        &self,
        params: &GetFavoriteGroupsParams,
    ) -> Result<Vec<FavoriteGroup>> {
        self.get("favorite/groups", Some(params)).await
    }

    /// PUT /favorite/group/{type}/{group}/{userId} – Update a favorite group.
    pub async fn save_favorite_group(
        &self,
        params: &SaveFavoriteGroupParams,
        user_id: &str,
    ) -> Result<FavoriteGroup> {
        self.put(
            &format!(
                "favorite/group/{}/{}/{}",
                params.r#type, params.group, user_id
            ),
            Some(params),
        )
        .await
    }

    /// DELETE /favorite/group/{type}/{group}/{userId} – Clear a favorite group.
    pub async fn clear_favorite_group(
        &self,
        r#type: &str,
        group: &str,
        user_id: &str,
    ) -> Result<serde_json::Value> {
        self.delete(
            &format!("favorite/group/{type}/{group}/{user_id}"),
            None::<&()>,
        )
        .await
    }
}
