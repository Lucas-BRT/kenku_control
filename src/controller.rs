use crate::{
    playlist::{
        response::{self, *},
        track::*,
    },
    soundboard::{response::*, sound::*},
};
use reqwest::{self, Client};
use std::{error::Error, string, time::Duration};

#[derive(Debug)]
pub enum KenkuState {
    Online,
    Offline,
}

fn build_client(milisseconds: u64) -> Client {
    return Client::builder()
        .timeout(Duration::from_millis(milisseconds))
        .build()
        .unwrap();
}

pub async fn check_kenku_server_state(ip: &String, port: &String) -> KenkuState {
    let client = build_client(5);
    let test_url = format!("http://{}:{}", ip, port);

    match client.get(test_url).send().await {
        Ok(_) => KenkuState::Online,
        Err(_) => KenkuState::Offline,
    }
}

#[derive(Debug)]
pub struct Controller {
    pub client: Client,
    pub ip: String,
    pub port: String,
    pub kenku_remote_state: KenkuState,
}



impl Controller {
    pub fn new(ip: String, port: String) -> Controller {
        let client = build_client(5);

        Controller {
            client: client,
            ip: ip,
            port: port,
            kenku_remote_state: KenkuState::Offline,
        }
    }

    pub async fn get_soundboard(&self) -> Result<SoundboardGetResponse, reqwest::Error> {
        let url = format!("http://{}:{}/v1/soundboard", self.ip, self.port);

        let response = self
            .client
            .get(url)
            .send()
            .await?
            .json::<SoundboardGetResponse>()
            .await?;

        Ok(response)
    }

    pub async fn get_soundboard_playback_state(&self) -> Result<SoundboardPlaybackResponse, reqwest::Error> {
        let url = format!("http://{}:{}/v1/soundboard/playback", self.ip, self.port);

        let response = self
            .client
            .get(url)
            .send()
            .await?
            .json::<SoundboardPlaybackResponse>()
            .await?;

        Ok(response)
    }

    pub async fn get_playlist(&self) -> Result<PlaylistGetResponse, reqwest::Error> {
        let url = format!("http://{}:{}/v1/playlist", self.ip, self.port);

        let response = self
            .client
            .get(url)
            .send()
            .await?
            .json::<PlaylistGetResponse>()
            .await?;

        Ok(response)
    }

    pub async fn get_playlist_playback_state(&self) -> Result<PlaylistPlayback, reqwest::Error> {

        let url = format!("http://{}:{}/v1/playlist/playback", self.ip, self.port);

        let response = self
            .client
            .get(url)
            .send()
            .await?
            .json::<PlaylistPlayback>()
            .await?;

        Ok(response)        

    }

    

}
