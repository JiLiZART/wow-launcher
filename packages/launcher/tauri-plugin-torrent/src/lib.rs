use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use commands::TorrentState;
pub use models::*;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("torrent")
        .setup(|app| {
            app.manage(TorrentState::new());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::add_torrent,
            commands::get_status,
            commands::pause_torrent,
            commands::resume_torrent,
            commands::remove_torrent,
            commands::list_torrents,
            commands::get_stats
        ])
        .build()
}
