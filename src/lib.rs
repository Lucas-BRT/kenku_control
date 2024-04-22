//! # Kenku Control
//!
//! `Kenku Control` is a API to manage your Kenku FM using Rust.

use reqwest::{self, Client, StatusCode};
use serde_json::json;
use std::{time::Duration, u16, u64};

pub mod playlist;
pub mod soundboard;

/// Represents the state of the Kenku server.
///
/// This enum has two variants:
/// * `Online`: Represents that the Kenku server is online and reachable.
/// * `Offline`: Represents that the Kenku server is offline or not reachable.
#[derive(Debug, PartialEq)]
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
/// * `port` - A 16 unsigned byte that holds the port number of the server.
/// * `delay_in_milisseconds` - A 16 unsigned byte that holds the time in milisseconds for the connection timeout.
///
/// # Returns
///
/// This function returns a `KenkuState` that represents the state of the server. If the GET request is successful, it returns `KenkuState::Online`. If the GET request fails, it returns `KenkuState::Offline`.
pub async fn check_kenku_server_state(ip: &str, port: u16, delay_in_milisseconds: u64) -> KenkuState {
    let client = build_client(delay_in_milisseconds);
    let test_url = format!("http://{}:{}", ip, port);

    match client.get(test_url).send().await {
        Ok(_) => KenkuState::Online,
        Err(_) => KenkuState::Offline,
    }
}

fn format_base_url(ip: &str, port: u16) -> String {
    format!("http://{}:{}/v1", ip, port)
}

fn process_get_command(command: KenkuGetCommand, ip: &str, port: u16) -> String {
    let base_url = format_base_url(ip, port);
    match command {
        KenkuGetCommand::Soundboard => format!("{}/soundboard", base_url),
        KenkuGetCommand::SoundboardPlayback => format!("{}/soundboard/playback", base_url),
        KenkuGetCommand::Playlist => format!("{}/playlist", base_url),
        KenkuGetCommand::PlaylistPlayback => format!("{}/playlist/playback", base_url)
    }
}

fn process_put_command(command: &KenkuPutCommand, ip: &str, port: u16) -> String {
    let base_url = format_base_url(ip, port);
    match command {
        KenkuPutCommand::PlaylistPlay => format!("{}/playlist/play", base_url),
        KenkuPutCommand::PlaylistPlaybackMute => format!("{}/playlist/playback/mute", base_url),
        KenkuPutCommand::PlaylistPlaybackPause => format!("{}/playlist/playback/pause", base_url),
        KenkuPutCommand::PlaylistPlaybackPlay => format!("{}/playlist/playback/play", base_url),
        KenkuPutCommand::PlaylistPlaybackRepeat => format!("{}/playlist/playback/repeat", base_url),
        KenkuPutCommand::PlaylistPlaybackShuffle => format!("{}/playlist/playback/shuffle", base_url),
        KenkuPutCommand::PlaylistPlaybackVolume => format!("{}/playlist/playback/volume", base_url),
        KenkuPutCommand::SoundboardPlay => format!("{}/soundboard/play", base_url),
        KenkuPutCommand::SoundboardStop => format!("{}/soundboard/stop", base_url),
    }
}

fn process_post_command(command: &KenkuPostCommand, ip: &str, port: u16) -> String {
    let base_url = format_base_url(ip, port);
    match command {
        KenkuPostCommand::PlaylistPlaybackNext => format!("{}/playlist/playback/next", base_url),
        KenkuPostCommand::PlaylistPlaybackPrevious => format!("{}/playlist/playback/previous", base_url),
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
fn process_url(command: &KenkuCommand, ip: &String, port: &String) -> String {
    match command {
        KenkuCommand::KenkuGet(get_command) => match get_command {
            KenkuGetCommand::Soundboard => format!("http://{}:{}/v1/soundboard", ip, port),
            KenkuGetCommand::SoundboardPlayback => {
                format!("http://{}:{}/v1/soundboard/playback", ip, port)
            }
            KenkuGetCommand::Playlist => format!("http://{}:{}/v1/playlist", ip, port),
            KenkuGetCommand::PlaylistPlayback => {
                format!("http://{}:{}/v1/playlist/playback", ip, port)
            }
        },
        KenkuCommand::KenkuPut(put_command) => match put_command {
            KenkuPutCommand::PlaylistPlay => format!("http://{}:{}/v1/playlist/play", ip, port),
            KenkuPutCommand::PlaylistPlaybackMute => {
                format!("http://{}:{}/v1/playlist/playback/mute", ip, port)
            }
            KenkuPutCommand::PlaylistPlaybackPause => {
                format!("http://{}:{}/v1/playlist/playback/pause", ip, port)
            }
            KenkuPutCommand::PlaylistPlaybackPlay => {
                format!("http://{}:{}/v1/playlist/playback/play", ip, port)
            }
            KenkuPutCommand::PlaylistPlaybackRepeat => {
                format!("http://{}:{}/v1/playlist/playback/repeat", ip, port)
            }
            KenkuPutCommand::PlaylistPlaybackShuffle => {
                format!("http://{}:{}/v1/playlist/playback/shuffle", ip, port)
            }
            KenkuPutCommand::PlaylistPlaybackVolume => {
                format!("http://{}:{}/v1/playlist/playback/volume", ip, port)
            }
            KenkuPutCommand::SoundboardPlay => format!("http://{}:{}/v1/soundboard/play", ip, port),
            KenkuPutCommand::SoundboardStop => format!("http://{}:{}/v1/soundboard/stop", ip, port),
        },
        KenkuCommand::KenkuPost(post_command) => match post_command {
            KenkuPostCommand::PlaylistPlaybackNext => {
                format!("http://{}:{}/v1/playlist/playback/next", ip, port)
            }
            KenkuPostCommand::PlaylistPlaybackPrevious => {
                format!("http://{}:{}/v1/playlist/playback/previous", ip, port)
            }
        },
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
    PlaylistPlaybackRepeat(playlist::Repeat),
}

/// Represents a command to be sent to the Kenku server.
///
/// The `KenkuCommand` enum is used to encapsulate different types of commands that can be sent to the Kenku server, such as GET, PUT, and POST commands.
///
/// # Variants
///
/// * `KenkuGet` - Represents a GET command with a specific `KenkuGetCommand` payload.
/// * `KenkuPut` - Represents a PUT command with a specific `KenkuPutCommand` payload.
/// * `KenkuPost` - Represents a POST command with a specific `KenkuPostCommand` payload.
pub enum KenkuCommand {
    KenkuGet(KenkuGetCommand),
    KenkuPut(KenkuPutCommand),
    KenkuPost(KenkuPostCommand),
}

/// Represents the different types of PUT commands that can be sent to the Kenku server.
///
/// The `KenkuPutCommand` enum is used to specify the type of PUT command to be sent to the Kenku server. It includes commands related to soundboard playback and playlist playback control.
///
/// # Variants
///
/// * `SoundboardPlay` - Represents a command to play a soundboard item.
/// * `SoundboardStop` - Represents a command to stop the playback of a soundboard item.
/// * `PlaylistPlay` - Represents a command to play a track in the playlist.
/// * `PlaylistPlaybackPlay` - Represents a command to resume playback of the current track in the playlist.
/// * `PlaylistPlaybackPause` - Represents a command to pause the playback of the current track in the playlist.
/// * `PlaylistPlaybackMute` - Represents a command to mute the playback of the current track in the playlist.
/// * `PlaylistPlaybackVolume` - Represents a command to adjust the volume of the playback of the current track in the playlist.
/// * `PlaylistPlaybackShuffle` - Represents a command to enable or disable shuffle mode for the playlist playback.
/// * `PlaylistPlaybackRepeat` - Represents a command to enable or disable repeat mode for the playlist playback.
pub enum KenkuPutCommand {
    SoundboardPlay,
    SoundboardStop,
    PlaylistPlay,
    PlaylistPlaybackPlay,
    PlaylistPlaybackPause,
    PlaylistPlaybackMute,
    PlaylistPlaybackVolume,
    PlaylistPlaybackShuffle,
    PlaylistPlaybackRepeat,
}

/// Represents the different types of POST commands that can be sent to the Kenku server.
///
/// The `KenkuPostCommand` enum is used to specify the type of POST command to be sent to the Kenku server. It includes commands related to playlist playback control.
///
/// # Variants
///
/// * `PlaylistPlaybackNext` - Represents a command to play the next track in the playlist.
/// * `PlaylistPlaybackPrevious` - Represents a command to play the previous track in the playlist.
pub enum KenkuPostCommand {
    PlaylistPlaybackNext,
    PlaylistPlaybackPrevious,
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
    SoundboardGet(soundboard::SoundboardGetResponse),
    SoundboardPlayback(soundboard::SoundboardPlaybackResponse),
    PlaylistGet(playlist::PlaylistGetResponse),
    PlaylistPlayback(playlist::PlaylistPlaybackResponse),
}

/// Sends a request to the Kenku server to play the current track in the playlist.
///
/// This function constructs a URL for the 'PlaylistPlaybackPlay' command, sends a PUT request to that URL, and returns the HTTP status code of the response.
///
/// # Arguments
///
/// * `controller` - A reference to a `Controller` struct, which includes a HTTP client, the IP address and port of the server, and the current state of the server.
///
/// # Returns
///
/// This function returns a `Result` that contains a `StatusCode` if the request was sent successfully, or a `reqwest::Error` if the request failed.
pub async fn playback_play(controller: &Controller) -> Result<StatusCode, reqwest::Error> {
    let command = &KenkuCommand::KenkuPut(KenkuPutCommand::PlaylistPlaybackPlay);
    let url = process_url(command, &controller.ip, &controller.port);

    let response = controller.client.put(url).send().await?.status();

    Ok(response)
}

/// Sends a request to the Kenku server to pause the current track in the playlist.
///
/// This function constructs a URL for the 'PlaylistPlaybackPause' command, sends a PUT request to that URL, and returns the HTTP status code of the response.
///
/// # Arguments
///
/// * `controller` - A reference to a `Controller` struct, which includes a HTTP client, the IP address and port of the server, and the current state of the server.
///
/// # Returns
///
/// This function returns a `Result` that contains a `StatusCode` if the request was sent successfully, or a `reqwest::Error` if the request failed.
pub async fn playback_pause(controller: &Controller) -> Result<StatusCode, reqwest::Error> {
    let command = &KenkuCommand::KenkuPut(KenkuPutCommand::PlaylistPlaybackPause);
    let url = process_url(command, &controller.ip, &controller.port);

    let response = controller.client.put(url).send().await?.status();

    Ok(response)
}

/// Sends a request to the Kenku server to play the next track in the playlist.
///
/// This function constructs a URL for the 'PlaylistPlaybackNext' command, sends a POST request to that URL, and returns the HTTP status code of the response.
///
/// # Arguments
///
/// * `controller` - A reference to a `Controller` struct, which includes a HTTP client, the IP address and port of the server, and the current state of the server.
///
/// # Returns
///
/// This function returns a `Result` that contains a `StatusCode` if the request was sent successfully, or a `reqwest::Error` if the request failed.
pub async fn playback_next(controller: &Controller) -> Result<StatusCode, reqwest::Error> {
    let command = &KenkuCommand::KenkuPost(KenkuPostCommand::PlaylistPlaybackNext);
    let url = process_url(command, &controller.ip, &controller.port);
    println!("{url}");
    let response = controller.client.post(url).send().await?.status();

    Ok(response)
}

/// Sends a PUT request to the Kenku server to mute or unmute the playlist.
///
/// This function takes a reference to a `Controller` and a boolean, constructs a URL and a JSON payload, and sends a PUT request to the Kenku server. The server's response status is returned.
///
/// # Arguments
///
/// * `controller` - A reference to a `Controller` struct, which includes a HTTP client, the IP address and port of the server, and the current state of the server.
/// * `mute` - A boolean that specifies whether to mute (`true`) or unmute (`false`) the playlist.
///
/// # Returns
///
/// This function returns a `Result` that contains a `StatusCode`, if the request was sent successfully, or a `reqwest::Error`, if the request failed.
pub async fn playback_mute(
    controller: &Controller,
    mute: bool,
) -> Result<StatusCode, reqwest::Error> {
    let command = &KenkuCommand::KenkuPut(KenkuPutCommand::PlaylistPlaybackMute);
    let url = process_url(command, &controller.ip, &controller.port);
    let json = json!({"mute": mute});

    let response = controller
        .client
        .put(url)
        .header("content-type", "application/json")
        .json(&json)
        .send()
        .await?
        .status();

    Ok(response)
}

/// Changes the volume of the playlist.
///
/// This function takes a `Controller` and a floating point number representing the desired volume level.
/// It sends a PUT request to the server to change the volume of the playlist.
///
/// # Arguments
///
/// * `controller` - A reference to a `Controller` struct, which includes the HTTP client, the IP address and port of the server, and the current state of the server.
/// * `volume` - A floating point number representing the desired volume level. The value should be between 0.0 and 1.0, where 0.0 is mute and 1.0 is the maximum volume.
///
/// # Returns
///
/// This function returns a `Result` with a `StatusCode`. If the PUT request is successful, it returns `Ok(StatusCode)`. If the PUT request fails, it returns `Err(reqwest::Error)`.
pub async fn playback_volume(
    controller: &Controller,
    volume: f64,
) -> Result<StatusCode, reqwest::Error> {
    // Define the command as a volume command
    let command = &KenkuCommand::KenkuPut(KenkuPutCommand::PlaylistPlaybackVolume);

    // Construct the URL for the PUT request
    let url = process_url(command, &controller.ip, &controller.port);

    // Create the JSON payload for the request
    let json = json!({"volume": volume});

    // Send the PUT request to the server and get the response status
    let response = controller
        .client
        .put(url)
        .header("content-type", "application/json")
        .json(&json)
        .send()
        .await?
        .status();

    // Return the response status
    Ok(response)
}

/// Changes the shuffle state of the playlist.
///
/// This function takes a `Controller` and a boolean value representing the desired shuffle state.
/// It sends a PUT request to the server to change the shuffle state of the playlist.
///
/// # Arguments
///
/// * `controller` - A reference to a `Controller` struct, which includes the HTTP client, the IP address and port of the server, and the current state of the server.
/// * `shuffle` - A boolean value representing the desired shuffle state. If `true`, the tracks in the playlist will be played in a random order. If `false`, the tracks will be played in the order they appear in the playlist.
///
/// # Returns
///
/// This function returns a `Result` with a `StatusCode`. If the PUT request is successful, it returns `Ok(StatusCode)`. If the PUT request fails, it returns `Err(reqwest::Error)`.
pub async fn playback_shuffle(
    controller: &Controller,
    shuffle: bool,
) -> Result<StatusCode, reqwest::Error> {
    // Define the command as a shuffle command
    let command = &KenkuCommand::KenkuPut(KenkuPutCommand::PlaylistPlaybackShuffle);

    // Construct the URL for the PUT request
    let url = process_url(command, &controller.ip, &controller.port);

    // Create the JSON payload for the request
    let json = json!({"shuffle": shuffle});

    // Send the PUT request to the server and get the response status
    let response = controller
        .client
        .put(url)
        .header("content-type", "application/json")
        .json(&json)
        .send()
        .await?
        .status();

    // Return the response status
    Ok(response)
}

/// Sends a PUT request to the Kenku server to set the repeat mode of the playlist.
///
/// This function takes a reference to a `Controller` and a `Repeat` enum, constructs a URL and a JSON payload, and sends a PUT request to the Kenku server. The server's response status is returned.
///
/// # Arguments
///
/// * `controller` - A reference to a `Controller` struct, which includes a HTTP client, the IP address and port of the server, and the current state of the server.
/// * `repeat` - A `Repeat` enum, which specifies the repeat mode to set. It can be `Repeat::Track`, `Repeat::Playlist`, or `Repeat::Off`.
///
/// # Returns
///
/// This function returns a `Result` that contains a `StatusCode`, if the request was sent successfully, or a `reqwest::Error`, if the request failed.
pub async fn playback_repeat(
    controller: &Controller,
    repeat: playlist::Repeat,
) -> Result<StatusCode, reqwest::Error> {
    let command = &KenkuCommand::KenkuPut(KenkuPutCommand::PlaylistPlaybackRepeat);
    let url = process_url(command, &controller.ip, &controller.port);
    let json = json!({"repeat": repeat});

    println!("{url}");

    println!("{json}");

    let response = controller
        .client
        .put(url)
        .header("content-type", "application/json")
        .json(&json)
        .send()
        .await?
        .status();

    Ok(response)
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
            client,
            ip,
            port,
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
    pub async fn get_soundboard(&self) -> Result<soundboard::SoundboardGetResponse , reqwest::Error> {
        let url = process_url(
            &KenkuCommand::KenkuGet(KenkuGetCommand::Soundboard),
            &self.ip,
            &self.port,
        );
        let response = self
            .client
            .get(url)
            .send()
            .await?
            .json::<soundboard::SoundboardGetResponse>()
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
    ) -> Result<soundboard::SoundboardPlaybackResponse, reqwest::Error> {
        let url = process_url(
            &KenkuCommand::KenkuGet(KenkuGetCommand::SoundboardPlayback),
            &self.ip,
            &self.port,
        );
        let response = self
            .client
            .get(url)
            .send()
            .await?
            .json::<soundboard::SoundboardPlaybackResponse>()
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
    pub async fn get_playlist(&self) -> Result<playlist::PlaylistGetResponse, reqwest::Error> {
        let url = process_url(
            &KenkuCommand::KenkuGet(KenkuGetCommand::Playlist),
            &self.ip,
            &self.port,
        );
        let response = self
            .client
            .get(url)
            .send()
            .await?
            .json::<playlist::PlaylistGetResponse>()
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
    pub async fn get_playlist_playback(&self) -> Result<playlist::PlaylistPlaybackResponse, reqwest::Error> {
        let url = process_url(
            &KenkuCommand::KenkuGet(KenkuGetCommand::PlaylistPlayback),
            &self.ip,
            &self.port,
        );
        let response = self
            .client
            .get(url)
            .send()
            .await?
            .json::<playlist::PlaylistPlaybackResponse>()
            .await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::{process_url, KenkuCommand, KenkuGetCommand, KenkuPutCommand, KenkuPostCommand};
    const DEFAULT_IP_ADDRESS: &str = "127.0.0.1";
    const DEFAULT_PORT: u16 = 3333;

    #[test]
    fn get_soundboard_link_creation() {
        let command = KenkuCommand::KenkuGet(KenkuGetCommand::Soundboard);
        let url = process_url(&command, &DEFAULT_IP_ADDRESS.to_string(), &DEFAULT_PORT.to_string());
        let expected_url = format!("http://{}:{}/v1/soundboard", DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        assert_eq!(url, expected_url);
    }

    #[test]
    fn get_playlist_link_creation() {
        let command = KenkuCommand::KenkuGet(KenkuGetCommand::Playlist);
        let url = process_url(&command, &DEFAULT_IP_ADDRESS.to_string(), &DEFAULT_PORT.to_string());
        let expected_url = format!("http://{}:{}/v1/playlist", DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        assert_eq!(url, expected_url);
    }

    #[test]
    fn get_soundboard_playback_link_creation() {
        let command = KenkuCommand::KenkuGet(KenkuGetCommand::SoundboardPlayback);
        let url = process_url(&command, &DEFAULT_IP_ADDRESS.to_string(), &DEFAULT_PORT.to_string());
        let expected_url = format!("http://{}:{}/v1/soundboard/playback", DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        assert_eq!(url, expected_url);
    }

    #[test]
    fn get_playlist_playback_link_creation() {
        let command = KenkuCommand::KenkuGet(KenkuGetCommand::PlaylistPlayback);
        let url = process_url(&command, &DEFAULT_IP_ADDRESS.to_string(), &DEFAULT_PORT.to_string());
        let expected_url = format!("http://{}:{}/v1/playlist/playback", DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        assert_eq!(url, expected_url);
    }

    #[test]
    fn put_playlist_play_link_creation() {
        let command = KenkuCommand::KenkuPut(KenkuPutCommand::PlaylistPlay);
        let url = process_url(&command, &DEFAULT_IP_ADDRESS.to_string(), &DEFAULT_PORT.to_string());
        let expected_url = format!("http://{}:{}/v1/playlist/play", DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        assert_eq!(url, expected_url);
    }

    #[test]
    fn put_playlist_playback_mute_link_creation() {
        let command = KenkuCommand::KenkuPut(KenkuPutCommand::PlaylistPlaybackMute);
        let url = process_url(&command, DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        let expected_url = format!("http://{}:{}/v1/playlist/playback/mute", DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        assert_eq!(url, expected_url);
    }

    #[test]
    fn put_playlist_playback_pause_link_creation() {
        let command = KenkuCommand::KenkuPut(KenkuPutCommand::PlaylistPlaybackPause);
        let url = process_url(&command, DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        let expected_url = format!("http://{}:{}/v1/playlist/playback/pause", DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        assert_eq!(url, expected_url);
    }

    #[test]
    fn put_playlist_playback_play_link_creation() {
        let command = KenkuCommand::KenkuPut(KenkuPutCommand::PlaylistPlaybackPlay);
        let url = process_url(&command, DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        let expected_url = format!("http://{}:{}/v1/playlist/playback/play", DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        assert_eq!(url, expected_url);
    }

    #[test]
    fn put_playlist_playback_repeat_link_creation() {
        let command = KenkuCommand::KenkuPut(KenkuPutCommand::PlaylistPlaybackRepeat);
        let url = process_url(&command, DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        let expected_url = format!("http://{}:{}/v1/playlist/playback/repeat", DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        assert_eq!(url, expected_url);
    }

    #[test]
    fn put_playlist_playback_shuffle_link_creation() {
        let command = KenkuCommand::KenkuPut(KenkuPutCommand::PlaylistPlaybackShuffle);
        let url = process_url(&command, DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        let expected_url = format!("http://{}:{}/v1/playlist/playback/shuffle", DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        assert_eq!(url, expected_url);
    }

    #[test]
    fn put_playlist_playback_volume_link_creation() {
        let command = KenkuCommand::KenkuPut(KenkuPutCommand::PlaylistPlaybackVolume);
        let url = process_url(&command, DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        let expected_url = format!("http://{}:{}/v1/playlist/playback/volume", DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        assert_eq!(url, expected_url);
    }

    #[test]
    fn put_soundboard_play_link_creation() {
        let command = KenkuCommand::KenkuPut(KenkuPutCommand::SoundboardPlay);
        let url = process_url(&command, DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        let expected_url = format!("http://{}:{}/v1/soundboard/play", DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        assert_eq!(url, expected_url);
    }

    #[test]
    fn put_soundboard_stop_link_creation() {
        let command = KenkuCommand::KenkuPut(KenkuPutCommand::SoundboardStop);
        let url = process_url(&command, DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        let expected_url = format!("http://{}:{}/v1/soundboard/stop", DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        assert_eq!(url, expected_url);
    }

    #[test]
    fn post_playlist_playback_next_link_creation() {
        let command = KenkuCommand::KenkuPost(KenkuPostCommand::PlaylistPlaybackNext);
        let url = process_url(&command, DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        let expected_url = format!("http://{}:{}/v1/playlist/playback/next", DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        assert_eq!(url, expected_url);
    }

    #[test]
    fn post_playlist_playback_previous_link_creation() {
        let command = KenkuCommand::KenkuPost(KenkuPostCommand::PlaylistPlaybackPrevious);
        let url = process_url(&command, DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        let expected_url = format!("http://{}:{}/v1/playlist/playback/previous", DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        assert_eq!(url, expected_url);
    }

}



