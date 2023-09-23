use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Soundboards {
    pub id: String,
    pub sounds: Vec<String>,
    pub background: String,
    pub title: String,
}

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


