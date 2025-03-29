use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingRequest {
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingResponse {
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TorrentInfo {
    pub name: String,
    pub hash: String,
    pub progress: f64,
    pub status: String,
    pub download_speed: u64,
    pub upload_speed: u64,
    pub peers: u32,
    pub size: u64,
    pub downloaded: u64,
    pub uploaded: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddTorrentRequest {
    pub magnet_url: String,
    pub download_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TorrentStats {
    pub total_downloaded: u64,
    pub total_uploaded: u64,
    pub active_torrents: u32,
    pub total_peers: u32,
}
