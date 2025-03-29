use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::TorrentExt;
use crate::models::{AddTorrentRequest, TorrentInfo, TorrentStats};
use rusty_torrent::TorrentClient;
use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::State;

pub struct TorrentState {
    client: Arc<Mutex<TorrentClient>>,
}

impl TorrentState {
    pub fn new() -> Self {
        Self {
            client: Arc::new(Mutex::new(TorrentClient::new())),
        }
    }
}

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.torrent().ping(payload)
}

#[tauri::command]
pub async fn add_torrent(
    state: State<'_, TorrentState>,
    request: AddTorrentRequest,
) -> Result<TorrentInfo> {
    let mut client = state.client.lock().await;
    let torrent = client
        .add_torrent(&request.magnet_url)
        .await
        .map_err(|e| crate::Error::AddTorrentError(e.to_string()))?;

    Ok(TorrentInfo {
        name: torrent.name,
        hash: torrent.info_hash,
        progress: 0.0,
        status: "downloading".to_string(),
        download_speed: 0,
        upload_speed: 0,
        peers: 0,
        size: torrent.size,
        downloaded: 0,
        uploaded: 0,
    })
}

#[tauri::command]
pub async fn get_status(state: State<'_, TorrentState>, hash: String) -> Result<TorrentInfo> {
    let client = state.client.lock().await;
    let status = client
        .get_torrent_status(&hash)
        .await
        .map_err(|e| crate::Error::StatusError(e.to_string()))?;

    Ok(TorrentInfo {
        name: status.name,
        hash: status.info_hash,
        progress: status.progress,
        status: status.state.to_string(),
        download_speed: status.download_speed,
        upload_speed: status.upload_speed,
        peers: status.peers,
        size: status.size,
        downloaded: status.downloaded,
        uploaded: status.uploaded,
    })
}

#[tauri::command]
pub async fn pause_torrent(state: State<'_, TorrentState>, hash: String) -> Result<()> {
    let mut client = state.client.lock().await;
    client
        .pause_torrent(&hash)
        .await
        .map_err(|e| crate::Error::StatusError(e.to_string()))
}

#[tauri::command]
pub async fn resume_torrent(state: State<'_, TorrentState>, hash: String) -> Result<()> {
    let mut client = state.client.lock().await;
    client
        .resume_torrent(&hash)
        .await
        .map_err(|e| crate::Error::StatusError(e.to_string()))
}

#[tauri::command]
pub async fn remove_torrent(state: State<'_, TorrentState>, hash: String) -> Result<()> {
    let mut client = state.client.lock().await;
    client
        .remove_torrent(&hash)
        .await
        .map_err(|e| crate::Error::StatusError(e.to_string()))
}

#[tauri::command]
pub async fn list_torrents(state: State<'_, TorrentState>) -> Result<Vec<TorrentInfo>> {
    let client = state.client.lock().await;
    let torrents = client
        .list_torrents()
        .await
        .map_err(|e| crate::Error::StatusError(e.to_string()))?;

    Ok(torrents
        .into_iter()
        .map(|t| TorrentInfo {
            name: t.name,
            hash: t.info_hash,
            progress: t.progress,
            status: t.state.to_string(),
            download_speed: t.download_speed,
            upload_speed: t.upload_speed,
            peers: t.peers,
            size: t.size,
            downloaded: t.downloaded,
            uploaded: t.uploaded,
        })
        .collect())
}

#[tauri::command]
pub async fn get_stats(state: State<'_, TorrentState>) -> Result<TorrentStats> {
    let client = state.client.lock().await;
    let stats = client
        .get_stats()
        .await
        .map_err(|e| crate::Error::StatusError(e.to_string()))?;

    Ok(TorrentStats {
        total_downloaded: stats.total_downloaded,
        total_uploaded: stats.total_uploaded,
        active_torrents: stats.active_torrents,
        total_peers: stats.total_peers,
    })
}
