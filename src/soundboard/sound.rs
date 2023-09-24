use serde::{Deserialize, Serialize};

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
