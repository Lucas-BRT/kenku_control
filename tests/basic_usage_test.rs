use kenku_control::{
    client::MockHttpClient,
    soundboard::{SoundboardGetResponse, Sounds},
    *,
};
use mockall::predicate::eq;
use rand::Rng;
use serde_json::json;
use uuid::Uuid;

use crate::playlist::PlaylistGetResponse;

#[tokio::test]
async fn play_a_random_track() {
    let playlist_response = PlaylistGetResponse {
        playlists: vec![],
        tracks: vec![playlist::Track {
            id: Uuid::new_v4().to_string(),
            url: "https://example.com/track.mp3".to_string(),
            title: "Example Track".to_string(),
            duration: Some(10),
            progress: Some(0),
        }],
    };

    let mut client = MockHttpClient::new();

    let expected_body = json!({ "id": playlist_response.tracks[0].id });
    client
        .expect_get::<PlaylistGetResponse>()
        .with(eq("http://127.0.0.1:3333/v1/playlist"))
        .times(1)
        .returning(move |_| Ok(playlist_response.clone()));

    client
        .expect_put()
        .with(
            eq("http://127.0.0.1:3333/v1/playlist/play"),
            eq(expected_body.clone()),
        )
        .times(1)
        .returning(move |_, _| Ok(()));

    let controller = Controller::from_client(client, DEFAULT_KENKU_REMOTE_ADDRESS);

    let playlists = controller
        .get_playlist()
        .await
        .expect("failed to get kenku playlists");

    if !playlists.tracks.is_empty() {
        let index = rand::thread_rng().gen_range(0..playlists.tracks.len());
        let track = &playlists.tracks[index];

        track.play(&controller).await.expect("failed to play track");
    }
}

#[tokio::test]
async fn play_a_random_sound() {
    let sound_id = Uuid::new_v4().to_string();
    let soundboard_response = SoundboardGetResponse {
        soundboards: vec![],
        sounds: vec![Sounds {
            id: sound_id.clone(),
            url: "https://example.com/sound.mp3".to_string(),
            title: "Example Sound".to_string(),
            _loop: false,
            volume: 1.0,
            fade_in: 0,
            fade_out: 0,
            duration: Some(10),
            progress: Some(0.0),
        }],
    };

    let expected_body = json!({ "id": soundboard_response.sounds[0].id });
    let mut client = MockHttpClient::new();
    client
        .expect_get::<SoundboardGetResponse>()
        .with(eq("http://127.0.0.1:3333/v1/soundboard"))
        .times(1)
        .returning(move |_| Ok(soundboard_response.clone()));

    client
        .expect_put()
        .with(
            eq("http://127.0.0.1:3333/v1/soundboard/play"),
            eq(expected_body.clone()),
        )
        .times(1)
        .returning(move |_, _| Ok(()));

    let controller = Controller::from_client(client, DEFAULT_KENKU_REMOTE_ADDRESS);
    let soundboards = controller
        .get_soundboard()
        .await
        .expect("failed to get kenku soundboards");

    if !soundboards.sounds.is_empty() {
        let index = rand::thread_rng().gen_range(0..soundboards.sounds.len());
        let sound = &soundboards.sounds[index];

        sound.play(&controller).await.expect("failed to play sound");
    }
}
