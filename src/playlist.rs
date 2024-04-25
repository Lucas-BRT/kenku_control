use super::*;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;

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
/// This struct is used to model the response from a playback request to a playlist.
///
/// # Fields
///
/// * `playing` - A boolean indicating whether the playlist is currently playing.
/// * `volume` - The volume level of the playback, represented as a floating point number between 0-1.
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

impl Track {
    /// Sends a request to the Kenku server to play a specific track in the playlist.
    ///
    /// This function constructs a URL for the 'PlaylistPlay' command, sends a PUT request to that URL with the track ID as JSON payload, and returns the HTTP status code of the response.
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the `Track` struct, which represents a track in the playlist.
    /// * `controller` - A reference to a `Controller` struct, which includes a HTTP client, the IP address and port of the server, and the current state of the server.
    ///
    /// # Returns
    ///
    /// This function returns a `Result` that contains a `StatusCode` if the request was sent successfully, or a `reqwest::Error` if the request failed.
    pub async fn play(&self, controller: &Controller) -> Result<StatusCode, reqwest::Error> {
        let command = &KenkuCommand::KenkuPut(KenkuPutCommand::PlaylistPlay);
        let url = process_url(command, controller.ip, controller.port);
        let json = json!({"id": self.id});

        let response = controller
            .client
            .put(url)
            .header("Content-Type", "application/json")
            .json(&json)
            .send()
            .await?
            .status();

        Ok(response)
    }
}

#[allow(unused)]
pub mod playback {

    use super::{Controller, KenkuCommand, KenkuPutCommand, KenkuPostCommand, StatusCode, process_url, json, playlist};

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
        let url = process_url(command, controller.ip, controller.port);
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
        let url = process_url(command, controller.ip, controller.port);
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
        let url = process_url(command, controller.ip, controller.port);
        let response = controller.client.post(url).send().await?.status();

        Ok(response)
    }

    /// Sends a request to the Kenku server to play the previous track in the playlist.
    ///
    /// This function constructs a URL for the 'PlaylistPlaybackPrevious' command, sends a POST request to that URL, and returns the HTTP status code of the response.
    ///
    /// # Arguments
    ///
    /// * `controller` - A reference to a `Controller` struct, which includes a HTTP client, the IP address and port of the server, and the current state of the server.
    ///
    /// # Returns
    ///
    /// This function returns a `Result` that contains a `StatusCode` if the request was sent successfully, or a `reqwest::Error` if the request failed.
    pub async fn playback_previous(controller: &Controller) -> Result<StatusCode, reqwest::Error> {
        let command = &KenkuCommand::KenkuPost(KenkuPostCommand::PlaylistPlaybackPrevious);
        let url = process_url(command, controller.ip, controller.port);
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
        let url = process_url(command, controller.ip, controller.port);
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
        let command = &KenkuCommand::KenkuPut(KenkuPutCommand::PlaylistPlaybackVolume);
        let url = process_url(command, controller.ip, controller.port);
        let json = json!({"volume": volume});

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
        let command = &KenkuCommand::KenkuPut(KenkuPutCommand::PlaylistPlaybackShuffle);
        let url = process_url(command, controller.ip, controller.port);
        let json = json!({"shuffle": shuffle});

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
        let url = process_url(command, controller.ip, controller.port);
        let json = json!({"repeat": repeat});

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
}

