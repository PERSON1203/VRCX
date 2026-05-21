use serde::Serialize;

use crate::client::VRChatClient;
use crate::error::Result;
use crate::models::notification::Notification;

// ---------------------------------------------------------------------------
// Params
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetNotificationsParams {
    pub n: Option<u32>,
    pub offset: Option<u32>,
    pub sent: Option<bool>,
    pub r#type: Option<String>,
    pub after: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SendInviteParams {
    pub message_slot: Option<u32>,
    pub instance_id: Option<String>,
    pub world_id: Option<String>,
    pub world_name: Option<String>,
    pub details: Option<String>,
    pub rsvp: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SendNotificationResponseParams {
    pub response_type: String,
    pub response_data: Option<String>,
}

// ---------------------------------------------------------------------------
// API impl
// ---------------------------------------------------------------------------

impl VRChatClient {
    // --- Notifications v1 ---

    /// GET /auth/user/notifications – List notifications (v1).
    pub async fn get_notifications(
        &self,
        params: &GetNotificationsParams,
    ) -> Result<Vec<Notification>> {
        self.get("auth/user/notifications", Some(params)).await
    }

    /// GET /auth/user/notifications?type=friendRequest&hidden=true – List hidden friend requests.
    pub async fn get_hidden_friend_requests(
        &self,
        n: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<Notification>> {
        #[derive(Serialize)]
        struct P {
            #[serde(rename = "type")]
            r#type: &'static str,
            hidden: bool,
            n: Option<u32>,
            offset: Option<u32>,
        }
        let params = P {
            r#type: "friendRequest",
            hidden: true,
            n,
            offset,
        };
        self.get("auth/user/notifications", Some(&params)).await
    }

    // --- Notifications v2 ---

    /// GET /notifications – List notifications (v2).
    pub async fn get_notifications_v2(
        &self,
        params: &GetNotificationsParams,
    ) -> Result<Vec<serde_json::Value>> {
        self.get("notifications", Some(params)).await
    }

    /// POST /notifications/{notificationId}/see – Mark a v2 notification as seen.
    pub async fn see_notification_v2(
        &self,
        notification_id: &str,
    ) -> Result<serde_json::Value> {
        self.post::<_, ()>(
            &format!("notifications/{notification_id}/see"),
            None,
        )
        .await
    }

    /// POST /notifications/{notificationId}/respond – Respond to a v2 notification.
    pub async fn send_notification_response(
        &self,
        notification_id: &str,
        params: &SendNotificationResponseParams,
    ) -> Result<serde_json::Value> {
        self.post(
            &format!("notifications/{notification_id}/respond"),
            Some(params),
        )
        .await
    }

    /// DELETE /notifications/{notificationId} – Delete a v2 notification.
    pub async fn hide_notification_v2(
        &self,
        notification_id: &str,
    ) -> Result<serde_json::Value> {
        self.delete(
            &format!("notifications/{notification_id}"),
            None::<&()>,
        )
        .await
    }

    // --- Notifications v1 mutation ---

    /// PUT /auth/user/notifications/{notificationId}/accept – Accept a friend request.
    pub async fn accept_friend_request_notification(
        &self,
        notification_id: &str,
    ) -> Result<serde_json::Value> {
        self.put::<_, ()>(
            &format!("auth/user/notifications/{notification_id}/accept"),
            None,
        )
        .await
    }

    /// PUT /auth/user/notifications/{notificationId}/hide – Hide a v1 notification.
    pub async fn hide_notification(
        &self,
        notification_id: &str,
    ) -> Result<serde_json::Value> {
        self.put::<_, ()>(
            &format!("auth/user/notifications/{notification_id}/hide"),
            None,
        )
        .await
    }

    /// PUT /auth/user/notifications/{notificationId}/see – Mark a v1 notification as seen.
    pub async fn see_notification(
        &self,
        notification_id: &str,
    ) -> Result<serde_json::Value> {
        self.put::<_, ()>(
            &format!("auth/user/notifications/{notification_id}/see"),
            None,
        )
        .await
    }

    // --- Invites ---

    /// POST /invite/{receiverUserId} – Send an invite.
    pub async fn send_invite(
        &self,
        receiver_user_id: &str,
        params: &SendInviteParams,
    ) -> Result<serde_json::Value> {
        self.post(&format!("invite/{receiver_user_id}"), Some(params))
            .await
    }

    /// POST /requestInvite/{receiverUserId} – Request an invite.
    pub async fn send_request_invite(
        &self,
        receiver_user_id: &str,
        params: &SendInviteParams,
    ) -> Result<serde_json::Value> {
        self.post(&format!("requestInvite/{receiver_user_id}"), Some(params))
            .await
    }

    /// POST /invite/{inviteId}/response – Respond to an invite.
    pub async fn send_invite_response(
        &self,
        invite_id: &str,
        params: &SendInviteParams,
    ) -> Result<serde_json::Value> {
        self.post(&format!("invite/{invite_id}/response"), Some(params))
            .await
    }
}
