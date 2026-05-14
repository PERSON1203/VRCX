
use crate::client::VRChatClient;
use crate::error::Result;
use crate::models::inventory::Prop;

// ---------------------------------------------------------------------------
// API impl
// ---------------------------------------------------------------------------

impl VRChatClient {
    /// GET /props/{propId} – Get a prop by ID.
    pub async fn get_prop(&self, prop_id: &str) -> Result<Prop> {
        self.get(&format!("props/{prop_id}"), None::<&()>).await
    }
}
