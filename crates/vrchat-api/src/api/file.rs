use crate::client::VRChatClient;
use crate::error::Result;
use crate::models::common::FileRecord;

// ---------------------------------------------------------------------------
// API impl
// ---------------------------------------------------------------------------

impl VRChatClient {
    /// GET /files – List files (used for avatar gallery, icons, etc.).
    pub async fn get_files(&self, params: &GetFilesParams) -> Result<Vec<FileRecord>> {
        self.get("files", Some(params)).await
    }

    /// PUT /file/{fileId}/{version}/file/start – Start a file-part upload.
    pub async fn start_file_upload(
        &self,
        file_id: &str,
        version: u32,
    ) -> Result<serde_json::Value> {
        self.put::<_, ()>(
            &format!("file/{file_id}/{version}/file/start"),
            None,
        )
        .await
    }

    /// PUT /file/{fileId}/{version}/file/finish – Finish a file-part upload.
    pub async fn finish_file_upload(
        &self,
        file_id: &str,
        version: u32,
        max_parts: u32,
    ) -> Result<serde_json::Value> {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Body {
            max_parts: u32,
        }
        self.put(
            &format!("file/{file_id}/{version}/file/finish"),
            Some(&Body { max_parts }),
        )
        .await
    }

    /// PUT /file/{fileId}/{version}/signature/start – Start a signature upload.
    pub async fn start_signature_upload(
        &self,
        file_id: &str,
        version: u32,
    ) -> Result<serde_json::Value> {
        self.put::<_, ()>(
            &format!("file/{file_id}/{version}/signature/start"),
            None,
        )
        .await
    }

    /// PUT /file/{fileId}/{version}/signature/finish – Finish a signature upload.
    pub async fn finish_signature_upload(
        &self,
        file_id: &str,
        version: u32,
        max_parts: u32,
    ) -> Result<serde_json::Value> {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Body {
            max_parts: u32,
        }
        self.put(
            &format!("file/{file_id}/{version}/signature/finish"),
            Some(&Body { max_parts }),
        )
        .await
    }

    /// PUT /files/order – Reorder files (avatar gallery order).
    pub async fn set_files_order(&self, ids: Vec<String>) -> Result<serde_json::Value> {
        #[derive(serde::Serialize)]
        struct Body {
            ids: Vec<String>,
        }
        self.put("files/order", Some(&Body { ids })).await
    }
}

// ---------------------------------------------------------------------------
// Params
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, serde::Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFilesParams {
    pub n: Option<u32>,
    pub offset: Option<u32>,
    pub tag: Option<String>,
    pub gallery_id: Option<String>,
}
