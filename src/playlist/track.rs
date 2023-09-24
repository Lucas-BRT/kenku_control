use serde::{Deserialize, Serialize};

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