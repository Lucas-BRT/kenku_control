use playlist::*;
use soundboard::*;
use reqwest::{self, Client};
use std::time::Duration;

mod soundboard {

    use serde::{Deserialize, Serialize};

    /// Represents the response from a GET request to a soundboard.
    ///
    /// This struct is used to model the response from a GET request to a soundboard. It includes a vector of `Soundboards` and a vector of `Sounds`.
    ///
    /// # Fields
    ///
    /// * `soundboards` - A vector of `Soundboards` representing the soundboards in the response.
    /// * `sounds` - A vector of `Sounds` representing the sounds in the response.
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct SoundboardGetResponse {
        pub soundboards: Vec<Soundboards>,
        pub sounds: Vec<Sounds>,
    }

    /// Represents the response from a playback request to a soundboard.
    ///
    /// This struct is used to model the response from a playback request to a soundboard. It includes a vector of `Sounds`.
    ///
    /// # Fields
    ///
    /// * `sounds` - A vector of `Sounds` representing the sounds in the response.
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct SoundboardPlaybackResponse {
        sounds: Vec<Sounds>,
    }

    /// Represents a soundboard.
    ///
    /// This struct is used to model a soundboard with its properties.
    ///
    /// # Fields
    ///
    /// * `id` - A unique identifier for the soundboard.
    /// * `sounds` - A vector of strings representing the sounds in the soundboard.
    /// * `background` - A string representing the background of the soundboard.
    /// * `title` - The title of the soundboard.
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Soundboards {
        pub id: String,
        pub sounds: Vec<String>,
        pub background: String,
        pub title: String,
    }

    /// Represents a sound.
    ///
    /// This struct is used to model a sound with its properties.
    ///
    /// # Fields
    ///
    /// * `id` - A unique identifier for the sound.
    /// * `url` - The URL where the sound file is located.
    /// * `title` - The title of the sound.
    /// * `_loop` - A boolean indicating whether the sound should loop.
    /// * `volume` - The volume level of the sound, represented as a floating point number.
    /// * `fade_in` - The duration of the fade-in effect at the start of the sound, in milliseconds.
    /// * `fade_out` - The duration of the fade-out effect at the end of the sound, in milliseconds.
    /// * `duration` - The total duration of the sound, in milliseconds. This is an optional field.
    /// * `progress` - The current progress of the sound, represented as a percentage. This is an optional field.
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Sounds {
        pub id: String,
        pub url: String,
        pub title: String,
        #[serde(rename = "loop")]
        pub _loop: bool,
        pub volume: f64,
        #[serde(rename = "fadeIn")]
        pub fade_in: u32,
        #[serde(rename = "fadeOut")]
        pub fade_out: u32,
        pub duration: Option<u32>,
        pub progress: Option<f64>,
    }
}

mod playlist {

    use serde::{Deserialize, Serialize};

    /// Represents the repeat mode for a playlist or track.
    ///
    /// This enum has three variants:
    /// * `Track`: Represents that the current track should be repeated.
    /// * `Playlist`: Represents that the entire playlist should be repeated.
    /// * `Off`: Represents that no repeat mode is active.
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum Repeat {
        #[serde(rename = "track")]
        Track,
        #[serde(rename = "playlist")]
        Playlist,
        #[serde(rename = "off")]
        Off,
    }

    /// Represents the response from a GET request to a playlist.
    ///
    /// This struct is used to model the response from a GET request to a playlist. It includes a vector of `Playlist` and a vector of `Track`.
    ///
    /// # Fields
    ///
    /// * `playlists` - A vector of `Playlist` representing the playlists in the response.
    /// * `tracks` - A vector of `Track` representing the tracks in the response.
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct PlaylistGetResponse {
        pub playlists: Vec<Playlist>,
        pub tracks: Vec<Track>,
    }

    /// Provides methods for `PlaylistGetResponse`.
    ///
    /// This implementation provides a method for getting the playlists from a `PlaylistGetResponse`.
    impl PlaylistGetResponse {
        /// Returns a reference to the vector of `Playlist` in the `PlaylistGetResponse`.
        ///
        /// # Returns
        ///
        /// This method returns a reference to the vector of `Playlist` in the `PlaylistGetResponse`.
        pub fn get_playlists(&self) -> &Vec<Playlist> {
            return &self.playlists;
        }
    }

    /// Represents the response from a playback request to a playlist.
    ///
    /// This struct is used to model the response from a playback request to a playlist. It includes the playback status and settings, as well as the current track and playlist.
    ///
    /// # Fields
    ///
    /// * `playing` - A boolean indicating whether the playlist is currently playing.
    /// * `volume` - The volume level of the playback, represented as a floating point number.
    /// * `muted` - A boolean indicating whether the playback is muted.
    /// * `shuffle` - A boolean indicating whether the tracks are being played in shuffle mode.
    /// * `repeat` - The current repeat mode, represented as a `Repeat` enum.
    /// * `tracks` - An optional vector of `Track` representing the current tracks in the playlist.
    /// * `playlist` - An optional `Playlist` representing the current playlist.
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct PlaylistPlaybackResponse {
        pub playing: bool,
        pub volume: f64,
        pub muted: bool,
        pub shuffle: bool,
        pub repeat: Repeat,
        pub tracks: Option<Vec<Track>>,
        pub playlist: Option<Playlist>,
    }

    /// Represents a playlist.
    ///
    /// This struct is used to model a playlist with its properties.
    ///
    /// # Fields
    ///
    /// * `id` - A unique identifier for the playlist.
    /// * `tracks` - An optional vector of strings representing the tracks in the playlist.
    /// * `background` - An optional string representing the background of the playlist.
    /// * `title` - The title of the playlist.
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Playlist {
        pub id: String,
        pub tracks: Option<Vec<String>>,
        pub background: Option<String>,
        pub title: String,
    }

    /// Represents a track.
    ///
    /// This struct is used to model a track with its properties.
    ///
    /// # Fields
    ///
    /// * `id` - A unique identifier for the track.
    /// * `url` - The URL where the track file is located.
    /// * `title` - The title of the track.
    /// * `duration` - The total duration of the track, in milliseconds. This is an optional field.
    /// * `progress` - The current progress of the track, in milliseconds. This is an optional field.
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Track {
        pub id: String,
        pub url: String,
        pub title: String,
        pub duration: Option<u32>,
        pub progress: Option<u32>,
    }
}

/// Represents the state of the Kenku server.
///
/// This enum has two variants:
/// * `Online`: Represents that the Kenku server is online and reachable.
/// * `Offline`: Represents that the Kenku server is offline or not reachable.
#[derive(Debug)]
pub enum KenkuState {
    Online,
    Offline,
}

/// Builds a new HTTP client with a specified timeout.
///
/// This function takes a timeout duration in milliseconds and returns a `reqwest::Client` with that timeout.
///
/// # Arguments
///
/// * `milisseconds` - The timeout duration in milliseconds.
///
/// # Returns
///
/// This function returns a `reqwest::Client` with the specified timeout.
///
/// # Panics
///
/// This function will panic if the client builder fails to build the client.
fn build_client(milisseconds: u64) -> Client {
    return Client::builder()
        .timeout(Duration::from_millis(milisseconds))
        .build()
        .unwrap();
}

/// Checks the state of the Kenku server.
///
/// This function takes an IP address and a port, makes a GET request to the server, and returns the server's state.
///
/// # Arguments
///
/// * `ip` - A string slice that holds the IP address of the server.
/// * `port` - A string slice that holds the port number of the server.
///
/// # Returns
///
/// This function returns a `KenkuState` that represents the state of the server. If the GET request is successful, it returns `KenkuState::Online`. If the GET request fails, it returns `KenkuState::Offline`.
///
/// # Examples
///
/// ```
/// let state = check_kenku_server_state(&"localhost".to_string(), &"8080".to_string()).await;
/// println!("{:?}", state);
/// ```
pub async fn check_kenku_server_state(ip: &String, port: &String) -> KenkuState {
    let client = build_client(5);
    let test_url = format!("http://{}:{}", ip, port);

    match client.get(test_url).send().await {
        Ok(_) => KenkuState::Online,
        Err(_) => KenkuState::Offline,
    }
}

/// Constructs a URL for a given command, IP address, and port.
///
/// This function takes a `KenkuGetCommand`, an IP address, and a port, and returns a URL that can be used to make a GET request to the soundboard or playlist API.
///
/// # Arguments
///
/// * `command` - A reference to a `KenkuGetCommand` enum, which specifies the type of request to make.
/// * `ip` - A string slice that holds the IP address of the server.
/// * `port` - A string slice that holds the port number of the server.
///
/// # Returns
///
/// This function returns a `String` that represents the constructed URL.
///
/// # Examples
///
/// ```
/// let command = KenkuGetCommand::Soundboard;
/// let ip = "localhost";
/// let port = "8080";
/// let url = process_url(&command, &ip, &port);
/// assert_eq!(url, "http://localhost:8080/v1/soundboard");
/// ```
fn process_url(command: &KenkuGetCommand, ip: &String, port: &String) -> String {
    match command {
        KenkuGetCommand::Soundboard => format!("http://{}:{}/v1/soundboard", ip, port),
        KenkuGetCommand::SoundboardPlayback => {
            format!("http://{}:{}/v1/soundboard/playback", ip, port)
        }
        KenkuGetCommand::Playlist => format!("http://{}:{}/v1/playlist", ip, port),
        KenkuGetCommand::PlaylistPlayback => format!("http://{}:{}/v1/playlist/playback", ip, port),
    }
}

/// Represents a command to control the playback of a playlist.
///
/// This enum has variants for each possible playback command, including play, pause, next, previous, mute, volume, shuffle, and repeat.
/// The `PlaylistPlaybackMute`, `PlaylistPlaybackVolume`, `PlaylistPlaybackShuffle`, and `PlaylistPlaybackRepeat` variants carry additional data.
#[derive(Debug)]
pub enum KenkuPlaybackCommand {
    PlaylistPlaybackPlay,
    PlaylistPlaybackPause,
    PlaylistPlaybackNext,
    PlaylistPlaybackPrevious,
    PlaylistPlaybackMute(bool),
    PlaylistPlaybackVolume(f64),
    PlaylistPlaybackShuffle(bool),
    PlaylistPlaybackRepeat(Repeat),
}

/// Represents a command to get the state of the soundboard or playlist.
///
/// This enum has variants for each possible get command, including getting the state of the soundboard, the playback state of the soundboard, the state of the playlist, and the playback state of the playlist.
#[derive(Debug)]
pub enum KenkuGetCommand {
    Soundboard,
    SoundboardPlayback,
    Playlist,
    PlaylistPlayback,
}

/// Represents a response from the soundboard or playlist API.
///
/// This enum can hold a response of any type, including `SoundboardGetResponse`, `SoundboardPlaybackResponse`, `PlaylistGetResponse`, and `PlaylistPlaybackResponse`.
pub enum KenkuResponse {
    SoundboardGet(SoundboardGetResponse),
    SoundboardPlayback(SoundboardPlaybackResponse),
    PlaylistGet(PlaylistGetResponse),
    PlaylistPlayback(PlaylistPlaybackResponse),
}

/// Represents a controller for the Kenku server.
///
/// This struct is used to model a controller for the Kenku server. It includes a HTTP client, the IP address and port of the server, and the current state of the server.
///
/// # Fields
///
/// * `client` - A `reqwest::Client` used to make HTTP requests to the server.
/// * `ip` - A string representing the IP address of the server.
/// * `port` - A string representing the port number of the server.
/// * `kenku_remote_state` - A `KenkuState` representing the current state of the server.
#[derive(Debug)]
pub struct Controller {
    pub client: Client,
    pub ip: String,
    pub port: String,
    pub kenku_remote_state: KenkuState,
}

/// Provides methods for `Controller`.
///
/// This implementation provides a method for creating a new `Controller`.
impl Controller {
    /// Creates a new `Controller`.
    ///
    /// This function takes an IP address and a port, builds a new HTTP client with a timeout of 20 milliseconds, and returns a new `Controller` with the client, IP address, port, and an initial server state of `KenkuState::Offline`.
    ///
    /// # Arguments
    ///
    /// * `ip` - A string that holds the IP address of the server.
    /// * `port` - A string that holds the port number of the server.
    ///
    /// # Returns
    ///
    /// This function returns a new `Controller` with the specified IP address, port, and an initial server state of `KenkuState::Offline`.
    pub fn new(ip: String, port: String) -> Controller {
        let client = build_client(20);

        Controller {
            client: client,
            ip: ip,
            port: port,
            kenku_remote_state: KenkuState::Offline,
        }
    }

    /// Sends a GET request to the soundboard API and returns a `SoundboardGetResponse`.
    ///
    /// This function constructs the URL for the request using the `process_url` function with the `KenkuGetCommand::Soundboard` command and the IP address and port of the server.
    ///
    /// # Returns
    ///
    /// A `Result` which is either a `SoundboardGetResponse` or a `reqwest::Error`.
    pub async fn get_soundboard(&self) -> Result<SoundboardGetResponse, reqwest::Error> {
        let url = process_url(&KenkuGetCommand::Soundboard, &self.ip, &self.port);
        let response = self
            .client
            .get(url)
            .send()
            .await?
            .json::<SoundboardGetResponse>()
            .await?;
        Ok(response)
    }

    /// Sends a GET request to the soundboard API to get the current playback state.
    ///
    /// This function constructs the URL for the request using the `process_url` function with the `KenkuGetCommand::SoundboardPlayback` command and the IP address and port of the server.
    ///
    /// # Returns
    ///
    /// A `Result` which is either a `SoundboardPlaybackResponse` or a `reqwest::Error`.
    pub async fn get_soundboard_playback(
        &self,
    ) -> Result<SoundboardPlaybackResponse, reqwest::Error> {
        let url = process_url(&KenkuGetCommand::SoundboardPlayback, &self.ip, &self.port);
        let response = self
            .client
            .get(url)
            .send()
            .await?
            .json::<SoundboardPlaybackResponse>()
            .await?;
        Ok(response)
    }

    /// Sends a GET request to the playlist API and returns a `PlaylistGetResponse`.
    ///
    /// This function constructs the URL for the request using the `process_url` function with the `KenkuGetCommand::Playlist` command and the IP address and port of the server.
    ///
    /// # Returns
    ///
    /// A `Result` which is either a `PlaylistGetResponse` or a `reqwest::Error`.
    pub async fn get_playlist(&self) -> Result<PlaylistGetResponse, reqwest::Error> {
        let url = process_url(&KenkuGetCommand::Playlist, &self.ip, &self.port);
        let response = self
            .client
            .get(url)
            .send()
            .await?
            .json::<PlaylistGetResponse>()
            .await?;
        Ok(response)
    }

    /// Sends a GET request to the playlist API to get the current playback state.
    ///
    /// This function constructs the URL for the request using the `process_url` function with the `KenkuGetCommand::PlaylistPlayback` command and the IP address and port of the server.
    ///
    /// # Returns
    ///
    /// A `Result` which is either a `PlaylistPlaybackResponse` or a `reqwest::Error`.
    pub async fn get_playlist_playback(&self) -> Result<PlaylistPlaybackResponse, reqwest::Error> {
        let url = process_url(&KenkuGetCommand::PlaylistPlayback, &self.ip, &self.port);
        let response = self
            .client
            .get(url)
            .send()
            .await?
            .json::<PlaylistPlaybackResponse>()
            .await?;
        Ok(response)
    }
}
