use kenku_control::{client::MockHttpClient, playlist::playback, *};
use rand::prelude::*;

#[tokio::test]
async fn pause_playlist_playback() {
    let client = MockHttpClient::new();
    let controller = Controller::from_client(client, DEFAULT_KENKU_REMOTE_ADDRESS);
    playback::playback_pause(&controller)
        .await
        .expect("failed to pause the playback.");
}

#[tokio::test]
async fn play_playlist_playback() {
    let client = MockHttpClient::new();
    let controller = Controller::from_client(client, DEFAULT_KENKU_REMOTE_ADDRESS);
    playback::playback_play(&controller)
        .await
        .expect("failed to play the playback.");
}

#[tokio::test]
async fn next_playlist_playback() {
    let client = MockHttpClient::new();
    let controller = Controller::from_client(client, DEFAULT_KENKU_REMOTE_ADDRESS);
    playback::playback_next(&controller)
        .await
        .expect("failed to go to next track on playback.");
}

#[tokio::test]
async fn previous_playlist_playback() {
    let client = MockHttpClient::new();
    let controller = Controller::from_client(client, DEFAULT_KENKU_REMOTE_ADDRESS);
    playback::playback_previous(&controller)
        .await
        .expect("failed to go to previous track on playback.");
}

#[tokio::test]
async fn mute_playlist_playback() {
    let client = MockHttpClient::new();
    let controller = Controller::from_client(client, DEFAULT_KENKU_REMOTE_ADDRESS);
    let is_muted = controller
        .get_playlist_playback()
        .await
        .expect("failed to get muted state.")
        .muted;
    playback::playback_mute(&controller, !is_muted)
        .await
        .expect("failed to change mute state.");
}

#[tokio::test]
async fn repeat_playlist_playback() {
    let client = MockHttpClient::new();
    let controller = Controller::from_client(client, DEFAULT_KENKU_REMOTE_ADDRESS);

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
    playback::playback_repeat(&controller, repeat)
        .await
        .expect("failed to change repeat state.");
}

#[tokio::test]
async fn shuffle_playlist_playback() {
    let client = MockHttpClient::new();
    let controller = Controller::from_client(client, DEFAULT_KENKU_REMOTE_ADDRESS);

    let is_shuffled = controller
        .get_playlist_playback()
        .await
        .expect("failed to get shuffle state.")
        .shuffle;
    playback::playback_shuffle(&controller, !is_shuffled)
        .await
        .expect("failed to change shuffle state.");
}

#[tokio::test]
async fn volume_playlist_playback() {
    let client = MockHttpClient::new();
    let controller = Controller::from_client(client, DEFAULT_KENKU_REMOTE_ADDRESS);

    let mut rng = rand::thread_rng();
    let volume: f64 = rng.gen_range(0..=10) as f64 / 10.0;
    playback::playback_volume(&controller, volume)
        .await
        .expect("failed to change playback volume.");
}
