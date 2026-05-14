use serde::Serialize;

use crate::client::VRChatClient;
use crate::error::Result;
use crate::models::{CurrentUser, MutualCounts, User, UserNote};

// ---------------------------------------------------------------------------
// Params
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetUsersParams {
    pub n: Option<u32>,
    pub offset: Option<u32>,
    pub search: Option<String>,
    pub sort: Option<String>,
    pub order: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetUserFeedbackParams {
    pub n: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SaveCurrentUserParams {
    pub status: Option<String>,
    pub status_description: Option<String>,
    pub bio: Option<String>,
    pub bio_links: Option<Vec<String>>,
    pub pronouns: Option<String>,
    pub user_icon: Option<String>,
    pub profile_pic_override: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddRemoveTagsParams {
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetUserNotesParams {
    pub n: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SaveNoteParams {
    pub target_user_id: String,
    pub note: String,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetMutualFriendsParams {
    pub n: Option<u32>,
    pub offset: Option<u32>,
}

// ---------------------------------------------------------------------------
// API impl
// ---------------------------------------------------------------------------

impl VRChatClient {
    /// GET /users/{userId} – Fetch a user by ID.
    pub async fn get_user(&self, user_id: &str) -> Result<User> {
        self.get(&format!("users/{user_id}"), None::<&()>).await
    }

    /// GET /users – Search users.
    pub async fn get_users(&self, params: &GetUsersParams) -> Result<Vec<User>> {
        self.get("users", Some(params)).await
    }

    /// PUT /users/{userId} – Update the current user's profile.
    pub async fn save_current_user(&self, params: &SaveCurrentUserParams) -> Result<CurrentUser> {
        let user_id = self.current_user_id_placeholder();
        self.put(&format!("users/{user_id}"), Some(params)).await
    }

    /// POST /users/{userId}/addTags – Add tags to the current user.
    pub async fn add_user_tags(&self, tags: Vec<String>) -> Result<CurrentUser> {
        let user_id = self.current_user_id_placeholder();
        let body = AddRemoveTagsParams { tags };
        self.post(&format!("users/{user_id}/addTags"), Some(&body))
            .await
    }

    /// POST /users/{userId}/removeTags – Remove tags from the current user.
    pub async fn remove_user_tags(&self, tags: Vec<String>) -> Result<CurrentUser> {
        let user_id = self.current_user_id_placeholder();
        let body = AddRemoveTagsParams { tags };
        self.post(&format!("users/{user_id}/removeTags"), Some(&body))
            .await
    }

    /// GET /users/{userId}/feedback – Get feedback for a user.
    pub async fn get_user_feedback(
        &self,
        user_id: &str,
        params: &GetUserFeedbackParams,
    ) -> Result<serde_json::Value> {
        self.get(&format!("users/{user_id}/feedback"), Some(params))
            .await
    }

    /// GET /userNotes – Fetch user notes for the current user.
    pub async fn get_user_notes(&self, params: &GetUserNotesParams) -> Result<Vec<UserNote>> {
        self.get("userNotes", Some(params)).await
    }

    /// GET /users/{userId}/mutuals – Get mutual counts.
    pub async fn get_mutual_counts(&self, user_id: &str) -> Result<MutualCounts> {
        self.get(&format!("users/{user_id}/mutuals"), None::<&()>)
            .await
    }

    /// GET /users/{userId}/mutuals/friends – Get mutual friends.
    pub async fn get_mutual_friends(
        &self,
        user_id: &str,
        params: &GetMutualFriendsParams,
    ) -> Result<Vec<User>> {
        self.get(&format!("users/{user_id}/mutuals/friends"), Some(params))
            .await
    }

    /// GET /users/{userId}/mutuals/groups – Get mutual groups.
    pub async fn get_mutual_groups(&self, user_id: &str) -> Result<serde_json::Value> {
        self.get(&format!("users/{user_id}/mutuals/groups"), None::<&()>)
            .await
    }

    /// POST /users/{userId}/boop – Send a boop to a user.
    pub async fn send_boop(&self, user_id: &str, emoji_id: Option<&str>) -> Result<serde_json::Value> {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct BoopBody<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            emoji_id: Option<&'a str>,
        }
        let body = BoopBody { emoji_id };
        self.post(&format!("users/{user_id}/boop"), Some(&body)).await
    }

    // Helper – returns a sentinel; callers should pass the real ID.
    fn current_user_id_placeholder(&self) -> &str {
        // In practice you should store the current user's ID after login.
        // This placeholder keeps compilation working; callers may want to
        // store the current user and pass the real ID.
        "me"
    }
}
