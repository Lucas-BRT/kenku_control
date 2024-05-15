//! # Kenku Control
//!
//! `Kenku Control` is a API to manage your Kenku FM using Rust.
use reqwest::{self, Client};
use std::{time::Duration, u16, u64};
use utils::*;

pub mod playlist;
pub mod soundboard;
pub mod utils;

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
    pub ip: &'static str,
    pub port: u16,
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
    pub fn new(ip: &'static str, port: u16) -> Controller {
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
    pub async fn get_soundboard(
        &self,
    ) -> Result<soundboard::SoundboardGetResponse, reqwest::Error> {
        let url = process_url(
            &KenkuCommand::KenkuGet(KenkuGetCommand::Soundboard),
            self.ip,
            self.port,
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
            self.ip,
            self.port,
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
            self.ip,
            self.port,
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
    pub async fn get_playlist_playback(
        &self,
    ) -> Result<playlist::PlaylistPlaybackResponse, reqwest::Error> {
        let url = process_url(
            &KenkuCommand::KenkuGet(KenkuGetCommand::PlaylistPlayback),
            self.ip,
            self.port,
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
mod kenku_commands {
    use super::{process_url, KenkuCommand, KenkuGetCommand, KenkuPostCommand, KenkuPutCommand};
    const DEFAULT_IP_ADDRESS: &str = "127.0.0.1";
    const DEFAULT_PORT: u16 = 3333;

    #[test]
    fn get_soundboard_link_creation() {
        let command = KenkuCommand::KenkuGet(KenkuGetCommand::Soundboard);
        let url = process_url(&command, DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        let expected_url = format!(
            "http://{}:{}/v1/soundboard",
            DEFAULT_IP_ADDRESS, DEFAULT_PORT
        );
        assert_eq!(url, expected_url);
    }

    #[test]
    fn get_playlist_link_creation() {
        let command = KenkuCommand::KenkuGet(KenkuGetCommand::Playlist);
        let url = process_url(&command, DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        let expected_url = format!("http://{}:{}/v1/playlist", DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        assert_eq!(url, expected_url);
    }

    #[test]
    fn get_soundboard_playback_link_creation() {
        let command = KenkuCommand::KenkuGet(KenkuGetCommand::SoundboardPlayback);
        let url = process_url(&command, DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        let expected_url = format!(
            "http://{}:{}/v1/soundboard/playback",
            DEFAULT_IP_ADDRESS, DEFAULT_PORT
        );
        assert_eq!(url, expected_url);
    }

    #[test]
    fn get_playlist_playback_link_creation() {
        let command = KenkuCommand::KenkuGet(KenkuGetCommand::PlaylistPlayback);
        let url = process_url(&command, DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        let expected_url = format!(
            "http://{}:{}/v1/playlist/playback",
            DEFAULT_IP_ADDRESS, DEFAULT_PORT
        );
        assert_eq!(url, expected_url);
    }

    #[test]
    fn put_playlist_play_link_creation() {
        let command = KenkuCommand::KenkuPut(KenkuPutCommand::PlaylistPlay);
        let url = process_url(&command, DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        let expected_url = format!(
            "http://{}:{}/v1/playlist/play",
            DEFAULT_IP_ADDRESS, DEFAULT_PORT
        );
        assert_eq!(url, expected_url);
    }

    #[test]
    fn put_playlist_playback_mute_link_creation() {
        let command = KenkuCommand::KenkuPut(KenkuPutCommand::PlaylistPlaybackMute);
        let url = process_url(&command, DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        let expected_url = format!(
            "http://{}:{}/v1/playlist/playback/mute",
            DEFAULT_IP_ADDRESS, DEFAULT_PORT
        );
        assert_eq!(url, expected_url);
    }

    #[test]
    fn put_playlist_playback_pause_link_creation() {
        let command = KenkuCommand::KenkuPut(KenkuPutCommand::PlaylistPlaybackPause);
        let url = process_url(&command, DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        let expected_url = format!(
            "http://{}:{}/v1/playlist/playback/pause",
            DEFAULT_IP_ADDRESS, DEFAULT_PORT
        );
        assert_eq!(url, expected_url);
    }

    #[test]
    fn put_playlist_playback_play_link_creation() {
        let command = KenkuCommand::KenkuPut(KenkuPutCommand::PlaylistPlaybackPlay);
        let url = process_url(&command, DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        let expected_url = format!(
            "http://{}:{}/v1/playlist/playback/play",
            DEFAULT_IP_ADDRESS, DEFAULT_PORT
        );
        assert_eq!(url, expected_url);
    }

    #[test]
    fn put_playlist_playback_repeat_link_creation() {
        let command = KenkuCommand::KenkuPut(KenkuPutCommand::PlaylistPlaybackRepeat);
        let url = process_url(&command, DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        let expected_url = format!(
            "http://{}:{}/v1/playlist/playback/repeat",
            DEFAULT_IP_ADDRESS, DEFAULT_PORT
        );
        assert_eq!(url, expected_url);
    }

    #[test]
    fn put_playlist_playback_shuffle_link_creation() {
        let command = KenkuCommand::KenkuPut(KenkuPutCommand::PlaylistPlaybackShuffle);
        let url = process_url(&command, DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        let expected_url = format!(
            "http://{}:{}/v1/playlist/playback/shuffle",
            DEFAULT_IP_ADDRESS, DEFAULT_PORT
        );
        assert_eq!(url, expected_url);
    }

    #[test]
    fn put_playlist_playback_volume_link_creation() {
        let command = KenkuCommand::KenkuPut(KenkuPutCommand::PlaylistPlaybackVolume);
        let url = process_url(&command, DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        let expected_url = format!(
            "http://{}:{}/v1/playlist/playback/volume",
            DEFAULT_IP_ADDRESS, DEFAULT_PORT
        );
        assert_eq!(url, expected_url);
    }

    #[test]
    fn put_soundboard_play_link_creation() {
        let command = KenkuCommand::KenkuPut(KenkuPutCommand::SoundboardPlay);
        let url = process_url(&command, DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        let expected_url = format!(
            "http://{}:{}/v1/soundboard/play",
            DEFAULT_IP_ADDRESS, DEFAULT_PORT
        );
        assert_eq!(url, expected_url);
    }

    #[test]
    fn put_soundboard_stop_link_creation() {
        let command = KenkuCommand::KenkuPut(KenkuPutCommand::SoundboardStop);
        let url = process_url(&command, DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        let expected_url = format!(
            "http://{}:{}/v1/soundboard/stop",
            DEFAULT_IP_ADDRESS, DEFAULT_PORT
        );
        assert_eq!(url, expected_url);
    }

    #[test]
    fn post_playlist_playback_next_link_creation() {
        let command = KenkuCommand::KenkuPost(KenkuPostCommand::PlaylistPlaybackNext);
        let url = process_url(&command, DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        let expected_url = format!(
            "http://{}:{}/v1/playlist/playback/next",
            DEFAULT_IP_ADDRESS, DEFAULT_PORT
        );
        assert_eq!(url, expected_url);
    }

    #[test]
    fn post_playlist_playback_previous_link_creation() {
        let command = KenkuCommand::KenkuPost(KenkuPostCommand::PlaylistPlaybackPrevious);
        let url = process_url(&command, DEFAULT_IP_ADDRESS, DEFAULT_PORT);
        let expected_url = format!(
            "http://{}:{}/v1/playlist/playback/previous",
            DEFAULT_IP_ADDRESS, DEFAULT_PORT
        );
        assert_eq!(url, expected_url);
    }
}
