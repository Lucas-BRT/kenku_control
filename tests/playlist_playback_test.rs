use kenku_control::{*, playlist::playback};

const DEFAULT_IP_ADDRESS: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 3333;

#[tokio::test]
async fn pause_playlist_playback() {
    let controller = Controller::new(DEFAULT_IP_ADDRESS, DEFAULT_PORT);
    let command = playback::playback_pause(&controller).await.unwrap();

    assert_eq!(command.is_success(), true);
}

#[tokio::test]
async fn play_playlist_playback() {
    let controller = Controller::new(DEFAULT_IP_ADDRESS, DEFAULT_PORT);
    let command = playback::playback_play(&controller).await.unwrap();

    assert_eq!(command.is_success(), true);
}

#[tokio::test]
async fn next_playlist_playback() {
    let controller = Controller::new(DEFAULT_IP_ADDRESS, DEFAULT_PORT);
    let command = playback::playback_next(&controller).await.unwrap();

    assert_eq!(command.is_success(), true);
}

