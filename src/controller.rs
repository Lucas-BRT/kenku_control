use crate::{
    playlist::{
        response::{self, *},
        track::*,
    },
    soundboard::{response::*, sound::*},
};
use reqwest::{self, Client};
use std::{error::Error, string, time::Duration};

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

#[derive(Debug)]
pub struct Controller {
    pub client: Client,
    pub ip: String,
    pub port: String,
    pub kenku_remote_state: KenkuState,
}

impl Controller {
    pub fn new(ip: String, port: String) -> Controller {
        let client = build_client(20);

        Controller {
            client: client,
            ip: ip,
            port: port,
            kenku_remote_state: KenkuState::Offline,
        }
    }

    /// Fetches the current state of the soundboard.
    ///
    /// # Arguments
    ///
    /// * `controller` - A reference to the Controller struct.
    ///
    /// # Examples
    ///
    /// ```
    /// let controller = Controller::new("localhost", "8080");
    /// let response = get_soundboard(&controller).await?;
    /// println!("{:?}", response);
    /// ```
    pub async fn get_soundboard(
        controller: &Controller,
    ) -> Result<SoundboardGetResponse, reqwest::Error> {
        let url = process_url(
            &KenkuGetCommand::Soundboard,
            &controller.ip,
            &controller.port,
        );
        let response = controller
            .client
            .get(url)
            .send()
            .await?
            .json::<SoundboardGetResponse>()
            .await?;
        Ok(response)
    }

    /// Fetches the current playback state of the soundboard.
    ///
    /// # Arguments
    ///
    /// * `controller` - A reference to the Controller struct.
    ///
    /// # Examples
    ///
    /// ```
    /// let controller = Controller::new("localhost", "8080");
    /// let response = get_soundboard_playback(&controller).await?;
    /// println!("{:?}", response);
    /// ```
    pub async fn get_soundboard_playback(
        controller: &Controller,
    ) -> Result<SoundboardPlaybackResponse, reqwest::Error> {
        let url = process_url(
            &KenkuGetCommand::SoundboardPlayback,
            &controller.ip,
            &controller.port,
        );
        let response = controller
            .client
            .get(url)
            .send()
            .await?
            .json::<SoundboardPlaybackResponse>()
            .await?;
        Ok(response)
    }

    /// Fetches the current state of the playlist.
    ///
    /// # Arguments
    ///
    /// * `controller` - A reference to the Controller struct.
    ///
    /// # Examples
    ///
    /// ```
    /// let controller = Controller::new("localhost", "8080");
    /// let response = get_playlist(&controller).await?;
    /// println!("{:?}", response);
    /// ```
    pub async fn get_playlist(
        controller: &Controller,
    ) -> Result<PlaylistGetResponse, reqwest::Error> {
        let url = process_url(&KenkuGetCommand::Playlist, &controller.ip, &controller.port);
        let response = controller
            .client
            .get(url)
            .send()
            .await?
            .json::<PlaylistGetResponse>()
            .await?;
        Ok(response)
    }

    /// Fetches the current playback state of the playlist.
    ///
    /// # Arguments
    ///
    /// * `controller` - A reference to the Controller struct.
    ///
    /// # Examples
    ///
    /// ```
    /// let controller = Controller::new("localhost", "8080");
    /// let response = get_playlist_playback(&controller).await?;
    /// println!("{:?}", response);
    /// ```
    pub async fn get_playlist_playback(
        controller: &Controller,
    ) -> Result<PlaylistPlaybackResponse, reqwest::Error> {
        let url = process_url(
            &KenkuGetCommand::PlaylistPlayback,
            &controller.ip,
            &controller.port,
        );
        let response = controller
            .client
            .get(url)
            .send()
            .await?
            .json::<PlaylistPlaybackResponse>()
            .await?;
        Ok(response)
    }
}
