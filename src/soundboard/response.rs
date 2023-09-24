use crate::soundboard::sound::{Soundboards, Sounds};
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
