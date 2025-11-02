use kenku_control::{client::MockHttpClient, *};
use rand::Rng;

#[tokio::test]
async fn play_a_random_track() {
    let client = MockHttpClient::new();
    let controller = Controller::from_client(client, DEFAULT_KENKU_REMOTE_ADDRESS);

    let playlists = controller
        .get_playlist()
        .await
        .expect("failed to get kenku playlists");
    let playlist_tracks = Some(playlists.tracks);

    if let Some(tracks) = playlist_tracks {
        let index = rand::thread_rng().gen_range(0..tracks.len());
        let track = &tracks[index];

        track.play(&controller).await.expect("failed to play track");
    }
}

#[tokio::test]
async fn play_a_random_sound() {
    let client = MockHttpClient::new();
    let controller = Controller::from_client(client, DEFAULT_KENKU_REMOTE_ADDRESS);
    let soundboards = controller
        .get_soundboard()
        .await
        .expect("failed to get kenku soundboards");
    let soundboards_sounds = Some(soundboards.sounds);

    if let Some(sounds) = soundboards_sounds {
        let index = rand::thread_rng().gen_range(0..sounds.len());
        let sound = &sounds[index];

        sound.play(&controller).await.expect("failed to play sound");
    }
}
