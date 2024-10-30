use kenku_control::*;
use rand::Rng;

const DEFAULT_IP_ADDRESS: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 3333;

#[tokio::test]
async fn play_a_random_track() {
    let controller = Controller::new(DEFAULT_IP_ADDRESS.to_string(), DEFAULT_PORT);
    let playlists = controller
        .get_playlist()
        .await
        .expect("failed to get kenku playlists");
    let playlist_tracks = Some(playlists.tracks);

    if let Some(tracks) = playlist_tracks {
        let index = rand::thread_rng().gen_range(0..tracks.len());
        let track = &tracks[index];

        let status_code = track.play(&controller).await.unwrap();
        assert!(status_code.is_success());
    }
}

#[tokio::test]
async fn play_a_random_sond() {
    let controller = Controller::new(DEFAULT_IP_ADDRESS.to_string(), DEFAULT_PORT);
    let soundboards = controller
        .get_soundboard()
        .await
        .expect("failed to get kenku soundboards");
    let soundboards_sounds = Some(soundboards.sounds);

    if let Some(sounds) = soundboards_sounds {
        let index = rand::thread_rng().gen_range(0..sounds.len());
        let sound = &sounds[index];

        let status_code = sound.play(&controller).await.unwrap();
        assert!(status_code.is_success())
    }
}
