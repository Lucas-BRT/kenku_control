use kenku_control::*;
use rand::Rng;

#[tokio::test]
async fn play_a_random_track() {
    let controller = Controller::default();
    let playlists = controller
        .get_playlist()
        .await
        .expect("failed to get kenku playlists");
    let playlist_tracks = Some(playlists.tracks);

    if let Some(tracks) = playlist_tracks {
        let index = rand::thread_rng().gen_range(0..tracks.len());
        let track = &tracks[index];

        let status_code = track.play(&controller).await;
    }
}

#[tokio::test]
async fn play_a_random_sond() {
    let controller = Controller::default();
    let soundboards = controller
        .get_soundboard()
        .await
        .expect("failed to get kenku soundboards");
    let soundboards_sounds = Some(soundboards.sounds);

    if let Some(sounds) = soundboards_sounds {
        let index = rand::thread_rng().gen_range(0..sounds.len());
        let sound = &sounds[index];

        let status_code = sound.play(&controller).await.unwrap();
    }
}
