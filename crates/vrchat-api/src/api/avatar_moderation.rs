use serde::Serialize;

use crate::client::VRChatClient;
use crate::error::Result;
use crate::models::AvatarModeration;

// ---------------------------------------------------------------------------
// Params
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendAvatarModerationParams {
    pub avatar_moderation_type: String,
    pub target_avatar_id: String,
}

// ---------------------------------------------------------------------------
// API impl
// ---------------------------------------------------------------------------

impl VRChatClient {
    /// GET /auth/user/avatarmoderations – List avatar moderations.
    pub async fn get_avatar_moderations(&self) -> Result<Vec<AvatarModeration>> {
        self.get("auth/user/avatarmoderations", None::<&()>).await
    }

    /// POST /auth/user/avatarmoderations – Send an avatar moderation.
    pub async fn send_avatar_moderation(
        &self,
        params: &SendAvatarModerationParams,
    ) -> Result<AvatarModeration> {
        self.post("auth/user/avatarmoderations", Some(params)).await
    }

    /// DELETE /auth/user/avatarmoderations?targetAvatarId=…&avatarModerationType=…
    pub async fn delete_avatar_moderation(
        &self,
        target_avatar_id: &str,
        avatar_moderation_type: &str,
    ) -> Result<serde_json::Value> {
        let endpoint = format!(
            "auth/user/avatarmoderations?targetAvatarId={}&avatarModerationType={}",
            urlencoding::encode(target_avatar_id),
            urlencoding::encode(avatar_moderation_type),
        );
        self.delete(&endpoint, None::<&()>).await
    }
}

// Tiny helper to URL-encode strings without adding a dependency.
mod urlencoding {
    pub fn encode(s: &str) -> String {
        url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
    }
}
