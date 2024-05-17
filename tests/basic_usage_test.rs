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

    match playlist_tracks {
        Some(tracks) => {
            let index = rand::thread_rng().gen_range(0..tracks.len());
            let track = &tracks[index];

            let status_code = track.play(&controller).await.unwrap();
            assert_eq!(status_code.is_success(), true);
        }
        None => (),
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

    match soundboards_sounds {
        Some(sounds) => {
            let index = rand::thread_rng().gen_range(0..sounds.len());
            let sound = &sounds[index];

            let status_code = sound.play(&controller).await.unwrap();
            assert_eq!(status_code.is_success(), true)
        }
        None => (),
    }
}
