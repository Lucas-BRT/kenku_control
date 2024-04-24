use kenku_control::{*, playlist::playback};
use rand::Rng;

const DEFAULT_IP_ADDRESS: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 3333;

#[tokio::test]
async fn play_a_randon_track() {
    let controller = Controller::new(DEFAULT_IP_ADDRESS, DEFAULT_PORT);
    let playlists = controller.get_playlist().await.unwrap();
    let playlist_tracks = Some(playlists.tracks);

    match playlist_tracks {
        Some(tracks) => {
            let index = rand::thread_rng().gen_range(0..tracks.len());
            let track = &tracks[index];

            let status_code = track.play(&controller).await.unwrap();
            assert_eq!(status_code.is_success(), true);
        },
        None => ()
    }
}
    
#[tokio::test]
async fn pause_playlist_playback() {
    let controller = Controller::new(DEFAULT_IP_ADDRESS, DEFAULT_PORT);
    let command = playback::playback_pause(&controller).await.unwrap();

    assert_eq!(command.is_success(), true);
}
