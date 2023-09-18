///! # kenku_control
///!
///! A library for manage your Kenku FM using remote acess.
use reqwest::{self, Client};

pub struct Controller {
    client: Client,
    ip: String,
    port: String,
}

impl Controller {
    pub fn new(ip: String, port: String) -> Controller {
        let client = Client::new();
        return Controller {
            client: client,
            ip: ip,
            port: port,
        };
    }

    pub fn get_ip(&self) -> String {
        return self.ip.clone();
    }

    pub fn get_port(&self) -> String {
        return self.port.clone();
    }

    pub fn get_client(&self) -> Client {
        return self.client.clone();
    }

    pub async fn get_soundboard(
        &self,
    ) -> Result<self::media::soundboard::SoundboardResponse, reqwest::Error> {
        let get_soundboard_url = format!("http://{}:{}/v1/soundboard", self.ip, self.port);

        let response = self
            .client
            .get(get_soundboard_url)
            .send()
            .await?
            .json::<self::media::soundboard::SoundboardResponse>()
            .await?;

        Ok(response)
    }

    pub async fn get_playlist(
        &self,
    ) -> Result<self::media::playlist::PlaylistResponse, reqwest::Error> {
        let get_playlist_url = format!("http://{}:{}/v1/playlist", self.ip, self.port);

        let response = self
            .client
            .get(get_playlist_url)
            .send()
            .await?
            .json::<self::media::playlist::PlaylistResponse>()
            .await?;

        Ok(response)
    }
}

pub mod media {

    pub mod soundboard {

        pub mod state {

            use serde::{Deserialize, Serialize};

            use crate::Controller;

            #[derive(Deserialize, Serialize, Debug, Clone)]
            pub struct Sounds {
                sounds: Vec<SoundboardPlaybackState>,
            }

            impl Sounds {
                pub fn get_playback_sounds(&self) -> Vec<SoundboardPlaybackState> {
                    return self.sounds.clone();
                }
            }

            #[derive(Deserialize, Serialize, Debug, Clone)]
            pub struct SoundboardPlaybackState {
                #[serde(rename = "id")]
                id: Option<String>,
                #[serde(rename = "url")]
                url: Option<String>,
                #[serde(rename = "title")]
                title: Option<String>,
                #[serde(rename = "loop")]
                loop_: Option<bool>,
                #[serde(rename = "volume")]
                volume: Option<f64>,
                #[serde(rename = "fadeIn")]
                fadein: Option<u32>,
                #[serde(rename = "fadeout")]
                fadeout: Option<u32>,
                #[serde(rename = "duration")]
                duration: Option<u32>,
                #[serde(rename = "progress")]
                progress: Option<f64>,
            }

            impl SoundboardPlaybackState {
                pub fn get_id(&self) -> Option<String> {
                    return self.id.clone();
                }

                pub fn get_url(&self) -> Option<String> {
                    return self.url.clone();
                }

                pub fn get_title(&self) -> Option<String> {
                    return self.title.clone();
                }

                pub fn get_loop(&self) -> Option<bool> {
                    return self.loop_.clone();
                }

                pub fn get_volume(&self) -> Option<f64> {
                    return self.volume.clone();
                }

                pub fn get_fadein(&self) -> Option<u32> {
                    return self.fadein.clone();
                }

                pub fn get_fadeout(&self) -> Option<u32> {
                    return self.fadeout.clone();
                }

                pub fn get_duration(&self) -> Option<u32> {
                    return self.duration.clone();
                }

                pub fn get_progress(&self) -> Option<f64> {
                    return self.progress.clone();
                }
            }

            pub async fn get_playback(controller: &Controller) -> Result<Sounds, reqwest::Error> {
                let playback_soundboard_url = format!(
                    "http://{}:{}/v1/soundboard/playback",
                    controller.get_ip(),
                    controller.get_port()
                );

                let response = controller
                    .get_client()
                    .get(playback_soundboard_url)
                    .send()
                    .await?
                    .json::<Sounds>()
                    .await?;

                Ok(response)
            }
        }

        use reqwest::StatusCode;
        use serde::{Deserialize, Serialize};
        use serde_json::json;

        use crate::Controller;

        #[derive(Deserialize, Serialize, Debug, Clone)]
        pub struct SoundboardResponse {
            #[serde(rename = "soundboards")]
            soundboards: Vec<Soundboard>,
            #[serde(rename = "sounds")]
            sounds: Vec<Sound>,
        }

        impl SoundboardResponse {
            pub fn get_soundboards(&self) -> Vec<Soundboard> {
                return self.soundboards.clone();
            }

            pub fn get_sounds(&self) -> Vec<Sound> {
                return self.sounds.clone();
            }
        }

        #[derive(Deserialize, Serialize, Debug, Clone)]
        pub struct Soundboard {
            #[serde(rename = "id")]
            id: String,
            #[serde(rename = "background")]
            background: String,
            #[serde(rename = "title")]
            title: String,
            #[serde(rename = "sounds")]
            sounds: Vec<String>,
        }

        #[derive(Deserialize, Serialize, Debug, Clone)]
        pub struct Sound {
            #[serde(rename = "id")]
            pub id: String,
            #[serde(rename = "url")]
            pub url: String,
            #[serde(rename = "title")]
            pub title: String,
            #[serde(rename = "loop")]
            pub loop_: bool,
            #[serde(rename = "volume")]
            pub volume: f32,
            #[serde(rename = "fadeIn")]
            pub fadein: u32,
            #[serde(rename = "fadeOut")]
            pub fadeout: u32,
        }

        impl Sound {
            pub async fn play(
                &self,
                controller: &Controller,
            ) -> Result<StatusCode, reqwest::Error> {
                let play_soundboard_url = format!(
                    "http://{}:{}/v1/soundboard/play",
                    controller.get_ip(),
                    controller.get_port()
                );
                let json_id = json!({ "id" : self.id });

                let response = controller
                    .get_client()
                    .put(play_soundboard_url)
                    .header("Content-Type", "application/json")
                    .json(&json_id)
                    .send()
                    .await?
                    .status();

                Ok(response)
            }

            pub async fn stop(
                &self,
                controller: &Controller,
            ) -> Result<StatusCode, reqwest::Error> {
                let stop_soundboard_url = format!(
                    "http://{}:{}/v1/soundboard/stop",
                    controller.get_ip(),
                    controller.get_port()
                );
                let json_id = json!({ "id" : self.id });

                let response = controller
                    .get_client()
                    .put(stop_soundboard_url)
                    .header("Content-Type", "application/json")
                    .json(&json_id)
                    .send()
                    .await?
                    .status();

                Ok(response)
            }
        }
    }

    pub mod playlist {

        use crate::Controller;
        use reqwest::StatusCode;
        use serde::{Deserialize, Serialize};
        use serde_json::json;

        mod state {

            use serde::{Deserialize, Serialize};

            #[derive(Deserialize, Serialize, Debug)]
            pub enum Repeat {
                #[serde(rename = "track")]
                Track,
                #[serde(rename = "playlist")]
                Playlist,
                #[serde(rename = "off")]
                Off,
            }

            #[derive(Deserialize, Serialize, Debug)]
            pub struct PlaylistPlaybackState {
                playing: bool,
                volume: f64,
                muted: bool,
                shuffle: bool,
                repeat: Repeat,
                track: Option<Track>,
                playlist: Option<Playlist>,
            }

            #[derive(Deserialize, Serialize, Debug)]
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

            #[derive(Deserialize, Serialize, Debug)]
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
            pub async fn play(
                &self,
                controller: &Controller,
            ) -> Result<StatusCode, reqwest::Error> {
                let play_soundboard_url = format!(
                    "http://{}:{}/v1/soundboard/play",
                    controller.get_ip(),
                    controller.get_port()
                );
                let json_id = json!({ "id" : self.id });

                let response = controller
                    .get_client()
                    .put(play_soundboard_url)
                    .header("Content-Type", "application/json")
                    .json(&json_id)
                    .send()
                    .await?
                    .status();

                Ok(response)
            }

            pub async fn stop(
                &self,
                controller: &Controller,
            ) -> Result<StatusCode, reqwest::Error> {
                let stop_soundboard_url = format!(
                    "http://{}:{}/v1/soundboard/stop",
                    controller.get_ip(),
                    controller.get_port()
                );
                let json_id = json!({ "id" : self.id });

                let response = controller
                    .get_client()
                    .put(stop_soundboard_url)
                    .header("Content-Type", "application/json")
                    .json(&json_id)
                    .send()
                    .await?
                    .status();

                Ok(response)
            }
        }
    }
}
