use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Playlist {
    pub id: String,
    pub tracks: Option<Vec<String>>,
    pub background: Option<String>,
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Track {
    pub id: String,
    pub url: String,
    pub title: String,
    pub duration: Option<u32>,
    pub progress: Option<u32>,
}













