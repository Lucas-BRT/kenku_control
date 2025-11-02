use kenku_control::{client::MockHttpClient, utils::check_kenku_server_state, *};

#[tokio::test]
async fn kenku_remote_is_online() {
    let default_address = DEFAULT_KENKU_REMOTE_ADDRESS;
    let server_state = check_kenku_server_state(default_address).await;

    assert_eq!(server_state, KenkuState::Online);
}

#[tokio::test]
async fn get_playlists() {
    let client = MockHttpClient::new();
    let controller = Controller::from_client(client, DEFAULT_KENKU_REMOTE_ADDRESS);
    let playlist = controller.get_playlist().await;

    assert!(playlist.is_ok());
}

#[tokio::test]
async fn get_soundboards() {
    let client = MockHttpClient::new();
    let controller = Controller::from_client(client, DEFAULT_KENKU_REMOTE_ADDRESS);
    let soundboard = controller.get_soundboard().await;

    assert!(soundboard.is_ok());
}

#[tokio::test]
async fn get_playlist_playback() {
    let client = MockHttpClient::new();
    let controller = Controller::from_client(client, DEFAULT_KENKU_REMOTE_ADDRESS);
    let playlist_playback = controller.get_playlist_playback().await;

    assert!(playlist_playback.is_ok());
}

#[tokio::test]
async fn get_soundboard_playback() {
    let client = MockHttpClient::new();
    let controller = Controller::from_client(client, DEFAULT_KENKU_REMOTE_ADDRESS);
    let soundboard_playback = controller.get_soundboard_playback().await;

    assert!(soundboard_playback.is_ok());
}
