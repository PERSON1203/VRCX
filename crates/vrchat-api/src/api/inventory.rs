use serde::Serialize;

use crate::client::VRChatClient;
use crate::error::Result;
use crate::models::inventory::{InventoryItem, InventoryTemplate};

// ---------------------------------------------------------------------------
// Params
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInventoryItemsParams {
    pub n: Option<u32>,
    pub offset: Option<u32>,
    pub order: Option<String>,
    pub types: Option<String>,
    pub flags: Option<String>,
    pub not_flags: Option<String>,
    pub archived: Option<bool>,
    pub equip_slot: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EquipItemParams {
    pub equip_slot: String,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RedeemRewardParams {
    pub code: String,
}

// ---------------------------------------------------------------------------
// API impl
// ---------------------------------------------------------------------------

impl VRChatClient {
    /// GET /user/{userId}/inventory/{inventoryId} – Get a user inventory item.
    pub async fn get_user_inventory_item(
        &self,
        user_id: &str,
        inventory_id: &str,
    ) -> Result<InventoryItem> {
        self.get(
            &format!("user/{user_id}/inventory/{inventory_id}"),
            None::<&()>,
        )
        .await
    }

    /// GET /inventory/{inventoryId} – Get an inventory item by ID.
    pub async fn get_inventory_item(&self, inventory_id: &str) -> Result<InventoryItem> {
        self.get(&format!("inventory/{inventory_id}"), None::<&()>)
            .await
    }

    /// GET /inventory – List inventory items.
    pub async fn get_inventory_items(
        &self,
        params: &GetInventoryItemsParams,
    ) -> Result<Vec<InventoryItem>> {
        self.get("inventory", Some(params)).await
    }

    /// PUT /inventory/{inventoryId}/consume – Consume a bundle item.
    pub async fn consume_inventory_bundle(
        &self,
        inventory_id: &str,
    ) -> Result<serde_json::Value> {
        self.put::<_, ()>(&format!("inventory/{inventory_id}/consume"), None)
            .await
    }

    /// GET /inventory/template/{templateId} – Get an inventory template.
    pub async fn get_inventory_template(
        &self,
        inventory_template_id: &str,
    ) -> Result<InventoryTemplate> {
        self.get(
            &format!("inventory/template/{inventory_template_id}"),
            None::<&()>,
        )
        .await
    }

    /// POST /reward/redeem – Redeem a reward code.
    pub async fn redeem_reward(&self, code: &str) -> Result<serde_json::Value> {
        let body = RedeemRewardParams {
            code: code.to_owned(),
        };
        self.post("reward/redeem", Some(&body)).await
    }

    /// GET /inventory/global – Get global inventory.
    pub async fn get_global_inventory(&self) -> Result<serde_json::Value> {
        self.get("inventory/global", None::<&()>).await
    }

    /// PUT /inventory/{inventoryId}/equip – Equip an item.
    pub async fn equip_item(
        &self,
        inventory_id: &str,
        params: &EquipItemParams,
    ) -> Result<InventoryItem> {
        self.put(&format!("inventory/{inventory_id}/equip"), Some(params))
            .await
    }

    /// PUT /inventory/{inventoryId} – Archive an item.
    pub async fn archive_item(&self, inventory_id: &str) -> Result<InventoryItem> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Body {
            is_archived: bool,
        }
        self.put(
            &format!("inventory/{inventory_id}"),
            Some(&Body { is_archived: true }),
        )
        .await
    }

    /// PUT /inventory/{inventoryId} – Unarchive an item.
    pub async fn unarchive_item(&self, inventory_id: &str) -> Result<InventoryItem> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Body {
            is_archived: bool,
        }
        self.put(
            &format!("inventory/{inventory_id}"),
            Some(&Body { is_archived: false }),
        )
        .await
    }
}
