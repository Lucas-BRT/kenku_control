use kenku_control::{playlist::playback, *};
use rand::prelude::*;

const DEFAULT_IP_ADDRESS: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 3333;

#[tokio::test]
async fn pause_playlist_playback() {
    let controller = Controller::new(DEFAULT_IP_ADDRESS.to_string(), DEFAULT_PORT);
    let command = playback::playback_pause(&controller)
        .await
        .expect("failed to pause the playback.");

    assert_eq!(command.is_success(), true);
}

#[tokio::test]
async fn play_playlist_playback() {
    let controller = Controller::new(DEFAULT_IP_ADDRESS.to_string(), DEFAULT_PORT);
    let command = playback::playback_play(&controller)
        .await
        .expect("failed to play the playback.");

    assert_eq!(command.is_success(), true);
}

#[tokio::test]
async fn next_playlist_playback() {
    let controller = Controller::new(DEFAULT_IP_ADDRESS.to_string(), DEFAULT_PORT);
    let command = playback::playback_next(&controller)
        .await
        .expect("failed to go to next track on playback.");

    assert_eq!(command.is_success(), true);
}

#[tokio::test]
async fn previous_playlist_playback() {
    let controller = Controller::new(DEFAULT_IP_ADDRESS.to_string(), DEFAULT_PORT);
    let command = playback::playback_previous(&controller)
        .await
        .expect("failed to go to previous track on playback.");

    assert_eq!(command.is_success(), true);
}

#[tokio::test]
async fn mute_playlist_playback() {
    let controller = Controller::new(DEFAULT_IP_ADDRESS.to_string(), DEFAULT_PORT);
    let is_muted = controller
        .get_playlist_playback()
        .await
        .expect("failed to get muted state.")
        .muted;
    let command = playback::playback_mute(&controller, !is_muted)
        .await
        .expect("failed to change mute state.");

    assert_eq!(command.is_success(), true);
}

#[tokio::test]
async fn repeat_playlist_playback() {
    let controller = Controller::new(DEFAULT_IP_ADDRESS.to_string(), DEFAULT_PORT);
    let repeat_state = controller
        .get_playlist_playback()
        .await
        .expect("failed to get repeat state.")
        .repeat;
    let repeat = match repeat_state {
        playlist::Repeat::Track => playlist::Repeat::Playlist,
        playlist::Repeat::Playlist => playlist::Repeat::Off,
        playlist::Repeat::Off => playlist::Repeat::Track,
    };
    let command = playback::playback_repeat(&controller, repeat)
        .await
        .expect("failed to change repeat state.");

    assert_eq!(command.is_success(), true);
}

#[tokio::test]
async fn shuffle_playlist_playback() {
    let controller = Controller::new(DEFAULT_IP_ADDRESS.to_string(), DEFAULT_PORT);
    let is_shuffled = controller
        .get_playlist_playback()
        .await
        .expect("failed to get shuffle state.")
        .shuffle;
    let command = playback::playback_shuffle(&controller, !is_shuffled)
        .await
        .expect("failed to change shuffle state.");

    assert_eq!(command.is_success(), true);
}

#[tokio::test]
async fn volume_playlist_playback() {
    let controller = Controller::new(DEFAULT_IP_ADDRESS.to_string(), DEFAULT_PORT);
    let mut rng = rand::thread_rng();
    let volume: f64 = rng.gen_range(0..=10) as f64 / 10.0;
    let command = playback::playback_volume(&controller, volume)
        .await
        .expect("failed to change playback volume.");

    assert_eq!(command.is_success(), true);
}
