use crate::soundboard::sound::{Soundboards, Sounds};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SoundboardGetResponse {
    pub soundboards: Vec<Soundboards>,
    pub sounds: Vec<Sounds>,
}

impl SoundboardGetResponse {
    pub fn get_soundboards(&self) -> &Vec<Soundboards> {
        return &self.soundboards;
    }

    pub fn get_sounds(&self) -> &Vec<Sounds> {
        return &self.sounds;
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SoundboardPlaybackResponse { 
    sounds: Vec<Sounds>
}