use crate::playlist::track::{Playlist, Track};
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