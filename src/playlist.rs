use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::controller::Controller;

pub mod state {

    use reqwest::StatusCode;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use crate::controller::Controller;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub enum Repeat {
        #[serde(rename = "track")]
        Track,
        #[serde(rename = "playlist")]
        Playlist,
        #[serde(rename = "off")]
        Off,
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct PlaybackState {
        #[serde(rename = "playing")]
        playing: bool,
        #[serde(rename = "volume")]
        volume: f64,
        #[serde(rename = "muted")]
        muted: bool,
        #[serde(rename = "shuffle")]
        shuffle: bool,
        #[serde(rename = "repeat")]
        repeat: Repeat,
        #[serde(rename = "track")]
        track: Option<Track>,
        #[serde(rename = "playlist")]
        playlist: Option<Playlist>,
    }

    impl PlaybackState {
        pub fn get_playing_state(&self) -> bool {
            return self.playing.clone();
        }
        pub fn get_volume(&self) -> f64 {
            return self.volume.clone();
        }
        pub fn get_muted(&self) -> bool {
            return self.muted.clone();
        }
        pub fn get_shuffle(&self) -> bool {
            return self.shuffle.clone();
        }

        pub fn get_repeat(&self) -> Repeat {
            return self.repeat.clone();
        }
        pub fn get_track(&self) -> Option<Track> {
            return self.track.clone();
        }
        pub fn get_playlist(&self) -> Option<Playlist> {
            return self.playlist.clone();
        }

        pub async fn play(&self, controller: &Controller) -> Result<StatusCode, reqwest::Error> {
            let play_playback_url = format!(
                "http://{}:{}/v1/playlist/playback/play",
                controller.get_ip(),
                controller.get_port()
            );

            match self.get_track() {
                Some(track) => {
                    let json_id = json!({ "id" : track.get_id() });

                    let response = controller
                        .get_client()
                        .put(play_playback_url)
                        .header("Content-Type", "application/json")
                        .json(&json_id)
                        .send()
                        .await?
                        .status();

                    Ok(response)
                }
                None => Ok(StatusCode::NO_CONTENT),
            }
        }
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Track {
        #[serde(rename = "id")]
        id: String,
        #[serde(rename = "url")]
        url: String,
        #[serde(rename = "title")]
        title: String,
        #[serde(rename = "duration")]
        duration: u32,
        #[serde(rename = "progress")]
        progress: u32,
    }

    impl Track {
        pub fn get_id(&self) -> String {
            return self.id.clone();
        }
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Playlist {
        #[serde(rename = "id")]
        id: String,
        #[serde(rename = "title")]
        title: String,
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PlaylistResponse {
    #[serde(rename = "playlists")]
    playlists: Vec<Playlist>,
    #[serde(rename = "tracks")]
    tracks: Vec<Track>,
}

impl PlaylistResponse {
    pub fn get_playlists(&self) -> Vec<Playlist> {
        return self.playlists.clone();
    }

    pub fn get_tracks(&self) -> Vec<Track> {
        return self.tracks.clone();
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Playlist {
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "tracks")]
    tracks: Vec<String>,
    #[serde(rename = "background")]
    background: String,
    #[serde(rename = "title")]
    title: String,
}

impl Playlist {
    pub fn get_id(&self) -> String {
        return self.id.clone();
    }

    pub fn get_tracks(&self) -> Vec<String> {
        return self.tracks.clone();
    }

    pub fn get_background(&self) -> String {
        return self.background.clone();
    }

    pub fn get_title(&self) -> String {
        return self.title.clone();
    }

    pub async fn play(&self, controller: &Controller) -> Result<StatusCode, reqwest::Error> {
        let play_playlist_url = format!(
            "http://{}:{}/v1/playlist/play",
            controller.get_ip(),
            controller.get_port()
        );
        let json_id = json!({ "id" : self.id });

        let response = controller
            .get_client()
            .put(play_playlist_url)
            .header("Content-Type", "application/json")
            .json(&json_id)
            .send()
            .await?
            .status();

        Ok(response)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Track {
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "url")]
    url: String,
    #[serde(rename = "title")]
    title: String,
}

impl Track {
    pub fn get_id(&self) -> String {
        return self.id.clone();
    }

    pub fn get_url(&self) -> String {
        return self.url.clone();
    }

    pub fn get_title(&self) -> String {
        return self.title.clone();
    }

    pub async fn play(&self, controller: &Controller) -> Result<StatusCode, reqwest::Error> {
        let play_track_url = format!(
            "http://{}:{}/v1/playlist/play",
            controller.get_ip(),
            controller.get_port()
        );
        let json_id = json!({ "id" : self.id });

        let response = controller
            .get_client()
            .put(play_track_url)
            .header("Content-Type", "application/json")
            .json(&json_id)
            .send()
            .await?
            .status();

        Ok(response)
    }
}
