use serde::Serialize;

use crate::client::VRChatClient;
use crate::error::Result;
use crate::models::inventory::VRChatBalance;
use crate::models::common::FileRecord;

// ---------------------------------------------------------------------------
// Params
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReportUserParams {
    pub content_type: String,
    pub reason: String,
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBadgeParams {
    pub badge_id: String,
    pub hidden: Option<bool>,
    pub showcased: Option<bool>,
}

// ---------------------------------------------------------------------------
// API impl
// ---------------------------------------------------------------------------

impl VRChatClient {
    /// GET /file/{fileId} – Get file metadata.
    pub async fn get_file(&self, file_id: &str) -> Result<FileRecord> {
        self.get(&format!("file/{file_id}"), None::<&()>).await
    }

    /// DELETE /file/{fileId} – Delete a file.
    pub async fn delete_file(&self, file_id: &str) -> Result<serde_json::Value> {
        self.delete(&format!("file/{file_id}"), None::<&()>).await
    }

    /// POST /userNotes – Save a note about a user.
    pub async fn save_note(&self, target_user_id: &str, note: &str) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Body<'a> {
            target_user_id: &'a str,
            note: &'a str,
        }
        self.post(
            "userNotes",
            Some(&Body {
                target_user_id,
                note,
            }),
        )
        .await
    }

    /// POST /feedback/{userId}/user – Report a user.
    pub async fn report_user(
        &self,
        user_id: &str,
        params: &ReportUserParams,
    ) -> Result<serde_json::Value> {
        self.post(&format!("feedback/{user_id}/user"), Some(params))
            .await
    }

    /// GET /analysis/{fileId}/{version}/{variant} – Get file analysis results.
    pub async fn get_file_analysis(
        &self,
        file_id: &str,
        version: u32,
        variant: &str,
    ) -> Result<serde_json::Value> {
        self.get(
            &format!("analysis/{file_id}/{version}/{variant}"),
            None::<&()>,
        )
        .await
    }

    /// GET /user/{userId}/balance – Get VRChat credits balance.
    pub async fn get_vrchat_credits(&self, user_id: &str) -> Result<VRChatBalance> {
        self.get(&format!("user/{user_id}/balance"), None::<&()>)
            .await
    }

    /// DELETE /instances/{location} – Close an instance.
    pub async fn close_instance(
        &self,
        location: &str,
        hard_close: bool,
    ) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Body {
            hard_close: bool,
        }
        self.delete(
            &format!("instances/{location}"),
            Some(&Body { hard_close }),
        )
        .await
    }

    /// DELETE /users/{userId}/{worldId}/persist – Delete world persist data.
    pub async fn delete_world_persist_data(
        &self,
        user_id: &str,
        world_id: &str,
    ) -> Result<serde_json::Value> {
        self.delete(
            &format!("users/{user_id}/{world_id}/persist"),
            None::<&()>,
        )
        .await
    }

    /// GET /users/{userId}/{worldId}/persist/exists – Check if world persist data exists.
    pub async fn has_world_persist_data(
        &self,
        user_id: &str,
        world_id: &str,
    ) -> Result<bool> {
        self.get(
            &format!("users/{user_id}/{world_id}/persist/exists"),
            None::<&()>,
        )
        .await
    }

    /// PUT /users/{userId}/badges/{badgeId} – Update a badge.
    pub async fn update_badge(
        &self,
        user_id: &str,
        params: &UpdateBadgeParams,
    ) -> Result<serde_json::Value> {
        self.put(
            &format!("users/{user_id}/badges/{}", params.badge_id),
            Some(params),
        )
        .await
    }

    /// GET /visits – Get visit count.
    pub async fn get_visits(&self) -> Result<serde_json::Value> {
        self.get("visits", None::<&()>).await
    }
}
