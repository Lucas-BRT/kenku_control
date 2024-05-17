use kenku_control::{utils::check_kenku_server_state, *};
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    str::FromStr,
};

const DEFAULT_IP: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 3333;

fn get_default_address() -> SocketAddrV4 {
    SocketAddrV4::new(Ipv4Addr::from_str("127.0.0.1").unwrap(), 3333)
}

#[tokio::test]
async fn kenku_remote_is_online() {
    let default_address = get_default_address();
    let server_state = check_kenku_server_state(default_address).await;

    assert_eq!(server_state, KenkuState::Online);
}

#[tokio::test]
async fn get_playlists() {
    let controller = Controller::new(DEFAULT_IP.to_string(), DEFAULT_PORT);
    let playlist = controller.get_playlist().await;

    assert_eq!(playlist.is_ok(), true);
}

#[tokio::test]
async fn get_soundboards() {
    let controller = Controller::new(DEFAULT_IP.to_string(), DEFAULT_PORT);
    let soundboard = controller.get_soundboard().await;

    assert_eq!(soundboard.is_ok(), true);
}

#[tokio::test]
async fn get_playlist_playback() {
    let controller = Controller::new(DEFAULT_IP.to_string(), DEFAULT_PORT);
    let playlist_playback = controller.get_playlist_playback().await;

    assert_eq!(playlist_playback.is_ok(), true);
}

#[tokio::test]
async fn get_soundboard_playback() {
    let controller = Controller::new(DEFAULT_IP.to_string(), DEFAULT_PORT);
    let soundboard_playback = controller.get_soundboard_playback().await;

    assert_eq!(soundboard_playback.is_ok(), true);
}
