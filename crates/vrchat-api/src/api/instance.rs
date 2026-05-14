use serde::Serialize;

use crate::client::VRChatClient;
use crate::error::Result;
use crate::models::{Instance, InstanceShortName};

// ---------------------------------------------------------------------------
// Params
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateInstanceParams {
    pub world_id: String,
    pub r#type: String,
    pub region: Option<String>,
    pub owner_id: Option<String>,
    pub group_id: Option<String>,
    pub group_access_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInstanceShortNameParams {
    pub short_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SelfInviteParams {
    pub short_name: Option<String>,
}

// ---------------------------------------------------------------------------
// API impl
// ---------------------------------------------------------------------------

impl VRChatClient {
    /// GET /instances/{worldId}:{instanceId} – Get an instance.
    pub async fn get_instance(&self, world_id: &str, instance_id: &str) -> Result<Instance> {
        self.get(
            &format!("instances/{world_id}:{instance_id}"),
            None::<&()>,
        )
        .await
    }

    /// POST /instances – Create an instance.
    pub async fn create_instance(&self, params: &CreateInstanceParams) -> Result<Instance> {
        self.post("instances", Some(params)).await
    }

    /// GET /instances/{worldId}:{instanceId}/shortName – Get instance short name.
    pub async fn get_instance_short_name(
        &self,
        world_id: &str,
        instance_id: &str,
        params: &GetInstanceShortNameParams,
    ) -> Result<InstanceShortName> {
        self.get(
            &format!("instances/{world_id}:{instance_id}/shortName"),
            Some(params),
        )
        .await
    }

    /// GET /instances/s/{shortName} – Get instance from short name.
    pub async fn get_instance_from_short_name(&self, short_name: &str) -> Result<Instance> {
        self.get(&format!("instances/s/{short_name}"), None::<&()>)
            .await
    }

    /// POST /invite/myself/to/{worldId}:{instanceId} – Self invite.
    pub async fn self_invite(
        &self,
        world_id: &str,
        instance_id: &str,
        short_name: Option<&str>,
    ) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Body<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            short_name: Option<&'a str>,
        }
        let body = Body { short_name };
        self.post(
            &format!("invite/myself/to/{world_id}:{instance_id}"),
            Some(&body),
        )
        .await
    }
}
