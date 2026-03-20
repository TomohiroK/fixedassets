use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AssetPhoto {
    pub id: String,
    pub asset_id: String,
    /// Compressed image as data URL (e.g. "data:image/webp;base64,...")
    pub data_url: String,
    /// Smaller thumbnail data URL
    pub thumbnail_url: String,
    pub filename: String,
    pub size_bytes: u32,
    pub created_at: String,
}

impl AssetPhoto {
    pub fn new(asset_id: String, data_url: String, thumbnail_url: String, filename: String, size_bytes: u32) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            asset_id,
            data_url,
            thumbnail_url,
            filename,
            size_bytes,
            created_at: chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string(),
        }
    }
}
