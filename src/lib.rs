///! # kenku_control
///!
///! A library for manage your Kenku FM using remote acess.
use reqwest::{self, Client};

pub struct Controller {
    pub client: Client,
    pub ip: String,
    pub port: String,
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
}

pub mod media {

    pub mod soundboard {

        mod state {

            use serde::{Deserialize, Serialize};

            #[derive(Deserialize, Serialize, Debug)]
            pub struct SoundboardPlaybackState {
                #[serde(rename = "id")]
                id: String,
                #[serde(rename = "url")]
                url: String,
                #[serde(rename = "title")]
                title: String,
                #[serde(rename = "loop")]
                loop_: bool,
                #[serde(rename = "volume")]
                volume: f64,
                #[serde(rename = "fadeIn")]
                fadein: u32,
                #[serde(rename = "fadeout")]
                fadeout: u32,
                #[serde(rename = "duration")]
                duration: u32,
                #[serde(rename = "progress")]
                progress: u32,
            }
        }

        use serde::{Deserialize, Serialize};

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
    }

    pub mod playlist {

        use serde::{Deserialize, Serialize};

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

        #[derive(Deserialize, Serialize, Debug)]
        pub struct SoundboardResponse {
            pub playlists: Vec<Playlist>,
            pub tracks: Vec<Track>,
        }

        #[derive(Deserialize, Serialize, Debug)]
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

        #[derive(Deserialize, Serialize, Debug)]
        pub struct Track {
            #[serde(rename = "id")]
            id: String,
            #[serde(rename = "url")]
            url: String,
            #[serde(rename = "title")]
            title: String,
        }
    }
}
