use serde::Serialize;

use crate::client::VRChatClient;
use crate::error::Result;
use crate::models::notification::PlayerModeration;

// ---------------------------------------------------------------------------
// Params
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendPlayerModerationParams {
    pub moderated: String,
    pub r#type: String,
}

// ---------------------------------------------------------------------------
// API impl
// ---------------------------------------------------------------------------

impl VRChatClient {
    /// GET /auth/user/playermoderations – List player moderations.
    pub async fn get_player_moderations(&self) -> Result<Vec<PlayerModeration>> {
        self.get("auth/user/playermoderations", None::<&()>).await
    }

    /// POST /auth/user/playermoderations – Send a player moderation (block/mute/etc.).
    pub async fn send_player_moderation(
        &self,
        params: &SendPlayerModerationParams,
    ) -> Result<PlayerModeration> {
        self.post("auth/user/playermoderations", Some(params)).await
    }

    /// PUT /auth/user/unplayermoderate – Remove a player moderation.
    pub async fn delete_player_moderation(
        &self,
        params: &SendPlayerModerationParams,
    ) -> Result<serde_json::Value> {
        self.put("auth/user/unplayermoderate", Some(params)).await
    }
}
