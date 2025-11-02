use kenku_control::{
    client::MockHttpClient,
    playlist::{PlaylistGetResponse, PlaylistPlaybackResponse},
    soundboard::{SoundboardGetResponse, SoundboardPlaybackResponse},
    *,
};
use mockall::predicate::eq;

#[tokio::test]
async fn kenku_remote_is_online() {
    let mut client = MockHttpClient::new();
    client.expect_ping().times(1).returning(|| Ok(()));

    let mut controller = Controller::from_client(client, DEFAULT_KENKU_REMOTE_ADDRESS);
    controller.check_server().await;

    assert!(controller.server_state() == KenkuState::Online);
}

#[tokio::test]
async fn get_playlists() {
    let mut client = MockHttpClient::new();
    client
        .expect_get::<PlaylistGetResponse>()
        .with(eq("http://127.0.0.1:3333/v1/playlist"))
        .times(1)
        .returning(|_| Ok(PlaylistGetResponse::default()));

    let controller = Controller::from_client(client, DEFAULT_KENKU_REMOTE_ADDRESS);
    let playlist = controller.get_playlist().await;

    assert!(playlist.is_ok());
}

#[tokio::test]
async fn get_soundboards() {
    let mut client = MockHttpClient::new();
    client
        .expect_get::<SoundboardGetResponse>()
        .with(eq("http://127.0.0.1:3333/v1/soundboard"))
        .times(1)
        .returning(|_| Ok(SoundboardGetResponse::default()));

    let controller = Controller::from_client(client, DEFAULT_KENKU_REMOTE_ADDRESS);
    let soundboard = controller.get_soundboard().await;

    assert!(soundboard.is_ok());
}

#[tokio::test]
async fn get_playlist_playback() {
    let mut client = MockHttpClient::new();
    client
        .expect_get::<PlaylistPlaybackResponse>()
        .with(eq("http://127.0.0.1:3333/v1/playlist/playback"))
        .times(1)
        .returning(|_| Ok(PlaylistPlaybackResponse::default()));

    let controller = Controller::from_client(client, DEFAULT_KENKU_REMOTE_ADDRESS);
    let playlist_playback = controller.get_playlist_playback().await;

    assert!(playlist_playback.is_ok());
}

#[tokio::test]
async fn get_soundboard_playback() {
    let mut client = MockHttpClient::new();

    client
        .expect_get()
        .with(eq("http://127.0.0.1:3333/v1/soundboard/playback"))
        .times(1)
        .returning(|_| Ok(SoundboardPlaybackResponse::default()));

    let controller = Controller::from_client(client, DEFAULT_KENKU_REMOTE_ADDRESS);
    let soundboard_playback = controller.get_soundboard_playback().await;

    assert!(soundboard_playback.is_ok());
}
