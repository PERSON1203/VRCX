use serde::Serialize;

use crate::client::VRChatClient;
use crate::error::Result;
use crate::models::{FriendStatus, User};

// ---------------------------------------------------------------------------
// Params
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFriendsParams {
    pub n: Option<u32>,
    pub offset: Option<u32>,
    pub offline: Option<bool>,
}

// ---------------------------------------------------------------------------
// API impl
// ---------------------------------------------------------------------------

impl VRChatClient {
    /// GET /auth/user/friends – List friends.
    pub async fn get_friends(&self, params: &GetFriendsParams) -> Result<Vec<User>> {
        self.get("auth/user/friends", Some(params)).await
    }

    /// POST /user/{userId}/friendRequest – Send a friend request.
    pub async fn send_friend_request(&self, user_id: &str) -> Result<serde_json::Value> {
        self.post::<_, ()>(&format!("user/{user_id}/friendRequest"), None)
            .await
    }

    /// DELETE /user/{userId}/friendRequest – Cancel a friend request.
    pub async fn cancel_friend_request(&self, user_id: &str) -> Result<serde_json::Value> {
        self.delete(&format!("user/{user_id}/friendRequest"), None::<&()>)
            .await
    }

    /// DELETE /auth/user/friends/{userId} – Remove a friend.
    pub async fn delete_friend(&self, user_id: &str) -> Result<serde_json::Value> {
        self.delete(&format!("auth/user/friends/{user_id}"), None::<&()>)
            .await
    }

    /// GET /user/{userId}/friendStatus – Check friend status.
    pub async fn get_friend_status(&self, user_id: &str) -> Result<FriendStatus> {
        self.get(&format!("user/{user_id}/friendStatus"), None::<&()>)
            .await
    }
}
