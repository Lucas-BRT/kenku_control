use kenku_control::*;

const DEFAULT_IP_ADDRESS: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 3333;

#[tokio::test]
async fn kenku_remote_is_online() {
    let server_state = check_kenku_server_state(DEFAULT_IP_ADDRESS, DEFAULT_PORT, 10).await;
    assert_eq!(server_state, KenkuState::Online);
}

#[tokio::test]
async fn get_playlists() {
    let controller = Controller::new(DEFAULT_IP_ADDRESS, DEFAULT_PORT);
    let playlist = controller.get_playlist().await;
    assert_eq!(playlist.is_ok(), true);
}

#[tokio::test]
async fn get_soundboards() {
    let controller = Controller::new(DEFAULT_IP_ADDRESS, DEFAULT_PORT);
    let soundboard = controller.get_soundboard().await;
    assert_eq!(soundboard.is_ok(), true);
}

#[tokio::test]
async fn get_playlist_playback() {
    let controller = Controller::new(DEFAULT_IP_ADDRESS, DEFAULT_PORT);
    let playlist_playback = controller.get_playlist_playback().await;
    assert_eq!(playlist_playback.is_ok(), true);
}

#[tokio::test]
async fn get_soundboard_playback() {
    let controller = Controller::new(DEFAULT_IP_ADDRESS, DEFAULT_PORT);
    let soundboard_playback = controller.get_soundboard_playback().await;
    assert_eq!(soundboard_playback.is_ok(), true);
}
