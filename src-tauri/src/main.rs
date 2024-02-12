// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use log::{debug, info, LevelFilter};
use std::{fs::File, io::copy};

use tauri::{Builder, SystemTray, SystemTrayEvent};

const TORRENT_URL: &'static str = "https://api.sirus.su/sirus.torrent";
const TORRENT_KEY: &'static str = "sirius-client.torrent";

// async fn download_file_to(url: &str, file_name: &str) -> Result<()> {
//     // Send an HTTP GET request to the URL
//     let response = reqwest::get(url).await?;
//     // Create a new file to write the downloaded image to
//     let mut file = File::create(file_name)?;

//     // Create a cursor that wraps the response body
//     let mut content = Cursor::new(response.bytes().await?);
//     // Copy the content from the cursor to the file
//     copy(&mut content, &mut file)?;

//     Ok(())
// }

// // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// async fn greet(name: &str) -> String {
//     let exe_path = std::env::current_exe().unwrap().parent().unwrap();

//     format!("Hello, {}! You've been greeted from Rust!", name);
//     let file_path = exe_path.join(TORRENT_KEY);
//     let download_path = exe_path.join("wow");

//     download_file_to(TORRENT_URL, file_path).await;

//     let torrent = lib_rusty_torrent::torrent::Torrent::from_torrent_file(&file_path)
//         .await
//         .unwrap();

//     let mut files = lib_rusty_torrent::files::Files::new();

//     files.create_files(&torrent, &download_path).await;

//     let addresses = torrent.get_trackers().unwrap();
//     let (remote_hostname, remote_port) = ("tracker.opentrackr.org", 1337);
//     dbg!("{}:{}", remote_hostname, remote_port);

//     let mut tracker = lib_rusty_torrent::tracker::Tracker::new(
//         "0.0.0.0:61389".parse().unwrap(),
//         SocketAddr::V4(addresses[0]),
//     )
//     .await
//     .unwrap();

//     println!(
//         "Successfully connected to tracker {}:{}",
//         remote_hostname, remote_port
//     );
//     let connection_message = lib_rusty_torrent::tracker::ConnectionMessage::from_buffer(
//         &tracker
//             .send_message(&lib_rusty_torrent::tracker::ConnectionMessage::create_basic_connection())
//             .await,
//     );

//     dbg!("{:?}", connection_message);

//     let announce_message_response =
//         lib_rusty_torrent::tracker::AnnounceMessageResponse::from_buffer(
//             &tracker
//                 .send_message(&lib_rusty_torrent::tracker::AnnounceMessage::new(
//                     connection_message.connection_id,
//                     &torrent.get_info_hash(),
//                     "-MY0001-123456654321",
//                     torrent.get_total_length() as i64,
//                 ))
//                 .await,
//         );

//     dbg!("{:?}", announce_message_response);
//     println!("Found Peers");

//     // Creates an assumed peer connection to the `SocketAddr` given
//     let mut peer = match lib_rusty_torrent::peer::Peer::create_connection(
//         format!(
//             "{}:{}",
//             announce_message_response.ips[0], announce_message_response.ports[0]
//         )
//         .parse()
//         .unwrap(),
//     )
//     .await
//     {
//         Err(_) => return,
//         Ok(peer) => peer,
//     };

//     let num_pieces = torrent.info.pieces.len() / 20;
//     peer.handshake(&torrent).await.unwrap();
//     peer.keep_alive_until_unchoke().await.unwrap();

//     println!(
//         "Successfully Created Connection with peer: {}",
//         peer.peer_id
//     );

//     let mut len = 0;

//     for index in 0..num_pieces {
//         let piece = peer
//             .request_piece(
//                 index as u32,
//                 torrent.info.piece_length as u32,
//                 &mut len,
//                 torrent.get_total_length() as u32,
//             )
//             .await
//             .unwrap();

//         if torrent.check_piece(&piece, index as u32) {
//             files.write_piece(piece).await;
//         } else {
//             break;
//         }
//     }

//     peer.disconnect().await.unwrap();
// }

fn main() {
    let system_tray = SystemTray::new();

    Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                // app.hide();
                // app.show();
                println!("system tray received a left click");
            }
            SystemTrayEvent::RightClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a right click");
            }
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a double click");
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "hide" => {
                    println!("system tray received a item");
                }
                _ => {}
            },
            _ => {}
        })
        // .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
