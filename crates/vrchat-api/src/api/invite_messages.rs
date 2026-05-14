use serde::Serialize;

use crate::client::VRChatClient;
use crate::error::Result;
use crate::models::notification::InviteMessage;

// ---------------------------------------------------------------------------
// Params
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EditInviteMessageParams {
    pub message: String,
}

// ---------------------------------------------------------------------------
// API impl
// ---------------------------------------------------------------------------

impl VRChatClient {
    /// GET /message/{userId}/{messageType} – List invite messages.
    pub async fn get_invite_messages(
        &self,
        user_id: &str,
        message_type: &str,
    ) -> Result<Vec<InviteMessage>> {
        self.get(&format!("message/{user_id}/{message_type}"), None::<&()>)
            .await
    }

    /// PUT /message/{userId}/{messageType}/{slot} – Edit an invite message.
    pub async fn edit_invite_message(
        &self,
        user_id: &str,
        message_type: &str,
        slot: u32,
        params: &EditInviteMessageParams,
    ) -> Result<Vec<InviteMessage>> {
        self.put(
            &format!("message/{user_id}/{message_type}/{slot}"),
            Some(params),
        )
        .await
    }
}
