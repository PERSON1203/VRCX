use serde::Serialize;

use crate::client::VRChatClient;
use crate::error::Result;
use crate::models::inventory::Print;

// ---------------------------------------------------------------------------
// Params
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPrintsParams {
    pub n: Option<u32>,
    pub offset: Option<u32>,
}

// ---------------------------------------------------------------------------
// API impl
// ---------------------------------------------------------------------------

impl VRChatClient {
    /// GET /prints/user/{userId} – List prints for a user.
    pub async fn get_prints(&self, user_id: &str, params: &GetPrintsParams) -> Result<Vec<Print>> {
        self.get(&format!("prints/user/{user_id}"), Some(params))
            .await
    }

    /// GET /prints/{printId} – Get a print by ID.
    pub async fn get_print(&self, print_id: &str) -> Result<Print> {
        self.get(&format!("prints/{print_id}"), None::<&()>).await
    }

    /// DELETE /prints/{printId} – Delete a print.
    pub async fn delete_print(&self, print_id: &str) -> Result<serde_json::Value> {
        self.delete(&format!("prints/{print_id}"), None::<&()>)
            .await
    }
}
