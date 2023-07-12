use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UpStreamRouteStorageModel {
    pub remote_addr: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u32>,
    #[serde(rename = "isBackup")]
    pub is_backup: bool,
}
