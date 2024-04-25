use kenku_control::{playlist::playback, *};
use rand::prelude::*;

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

#[tokio::test]
async fn previous_playlist_playback() {
    let controller = Controller::new(DEFAULT_IP_ADDRESS, DEFAULT_PORT);
    let command = playback::playback_previous(&controller).await.unwrap();

    assert_eq!(command.is_success(), true);
}

#[tokio::test]
async fn mute_playlist_playback() {
    let controller = Controller::new(DEFAULT_IP_ADDRESS, DEFAULT_PORT);
    let is_muted = controller.get_playlist_playback().await.unwrap().muted;
    let command = playback::playback_mute(&controller, !is_muted).await.unwrap();

    assert_eq!(command.is_success(), true);
}

#[tokio::test]
async fn repeat_playlist_playback() {
    let controller = Controller::new(DEFAULT_IP_ADDRESS, DEFAULT_PORT);
    let repeat_state = controller.get_playlist_playback().await.unwrap().repeat;
    let repeat = match repeat_state {
        playlist::Repeat::Track => playlist::Repeat::Playlist,
        playlist::Repeat::Playlist => playlist::Repeat::Off,
        playlist::Repeat::Off => playlist::Repeat::Track
    };
    let command = playback::playback_repeat(&controller, repeat).await.unwrap();

    assert_eq!(command.is_success(), true);
}

#[tokio::test]
async fn shuffle_playlist_playback() {
    let controller = Controller::new(DEFAULT_IP_ADDRESS, DEFAULT_PORT);
    let is_shuffled = controller.get_playlist_playback().await.unwrap().shuffle;
    let command = playback::playback_shuffle(&controller, !is_shuffled).await.unwrap();

    assert_eq!(command.is_success(), true);
}

#[tokio::test]
async fn volume_playlist_playback() {
    let controller = Controller::new(DEFAULT_IP_ADDRESS, DEFAULT_PORT);
    let mut rng = rand::thread_rng();
    let volume: f64 = rng.gen_range(0..=10) as f64 / 10.0;
    let command = playback::playback_volume(&controller, volume).await.unwrap();

    assert_eq!(command.is_success(), true);
}
