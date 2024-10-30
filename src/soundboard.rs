/// all the content of Soundboard of Kenku FM
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::*;

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
#[serde_with::skip_serializing_none]
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
/// * `volume` - The volume level of the sound, represented as a floating point number between 0-1.
/// * `fade_in` - The duration of the fade-in effect at the start of the sound, in milliseconds.
/// * `fade_out` - The duration of the fade-out effect at the end of the sound, in milliseconds.
/// * `duration` - The total duration of the sound, in milliseconds. This is an optional field and only is need in playback response.
/// * `progress` - The current progress of the sound, starts in 0 and go to duration. This is an optional field and only is need in playback response.
#[serde_with::skip_serializing_none]
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

impl Sounds {
    /// Sends a request to the Kenku server to play a specific sound in the soundboard.
    ///
    /// This function constructs a URL for the 'SoundboardPlay' command, sends a PUT request to that URL with the track ID as JSON payload, and returns the HTTP status code of the response.
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the `Sound` struct, which represents a sound in the soundboard.
    /// * `controller` - A reference to a `Controller` struct, which includes a HTTP client, the IP address and port of the server, and the current state of the server.
    ///
    /// # Returns
    ///
    /// This function returns a `Result` that contains a `StatusCode` if the request was sent successfully, or a `reqwest::Error` if the request failed.
    pub async fn play(&self, controller: &Controller) -> Result<StatusCode, reqwest::Error> {
        let command = &KenkuCommand::KenkuPut(KenkuPutCommand::SoundboardPlay);

        let url = process_url(command, controller.address);
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

    /// Sends a request to the Kenku server to stop a specific sound in the soundboard.
    ///
    /// This function constructs a URL for the 'SoundboardPlay' command, sends a PUT request to that URL with the track ID as JSON payload, and returns the HTTP status code of the response.
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the `Sound` struct, which represents a sound in the soundboard.
    /// * `controller` - A reference to a `Controller` struct, which includes a HTTP client, the IP address and port of the server, and the current state of the server.
    ///
    /// # Returns
    ///
    /// This function returns a `Result` that contains a `StatusCode` if the request was sent successfully, or a `reqwest::Error` if the request failed.
    pub async fn stop(&self, controller: &Controller) -> Result<StatusCode, reqwest::Error> {
        let command = &KenkuCommand::KenkuPut(KenkuPutCommand::SoundboardStop);

        let url = process_url(command, controller.address);
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
