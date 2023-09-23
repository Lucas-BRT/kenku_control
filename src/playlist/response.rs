use crate::playlist::track::{Playlist, Track};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Repeat {
    #[serde(rename = "track")]
    Track,
    #[serde(rename = "playlist")]
    Playlist,
    #[serde(rename = "off")]
    Off,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlaylistGetResponse {
    pub playlists: Vec<Playlist>,
    pub tracks: Vec<Track>,
}

impl PlaylistGetResponse {
    pub fn get_playlists(&self) -> &Vec<Playlist> {
        return &self.playlists;
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlaylistPlayback {
    pub playing: bool,
    pub volume: f64,
    pub muted: bool,
    pub shuffle: bool,
    pub repeat: Repeat,
    pub tracks: Option<Vec<Track>>,
    pub playlist: Option<Playlist>,
}




