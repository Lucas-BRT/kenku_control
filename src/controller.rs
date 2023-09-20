use reqwest::{self, Client};
use std::time::Duration;

use crate::{playlist, soundboard};

#[derive(Debug)]
pub enum Kind {
    KenkuServerOffLine,
}

#[derive(Debug)]
pub struct Error {
    pub kind: Kind,
    pub mensage: String,
}

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
    client: Client,
    ip: String,
    port: String,
}

impl Controller {
    pub async fn new(ip: String, port: String) -> Result<Controller, Error> {
        let client = build_client(5);

        match check_kenku_server_state(&ip, &port).await {
            KenkuState::Online => Ok(Controller {
                client: client,
                ip: ip,
                port: port,
            }),

            KenkuState::Offline => Err(Error {
                kind: Kind::KenkuServerOffLine,
                mensage: String::from("Turn on kenku remote in Kenku FM."),
            }),
        }
    }

    pub fn get_ip(&self) -> String {
        return self.ip.clone();
    }

    pub fn get_port(&self) -> String {
        return self.port.clone();
    }

    pub fn get_client(&self) -> Client {
        return self.client.clone();
    }

    pub async fn get_soundboard(&self) -> Result<soundboard::SoundboardResponse, reqwest::Error> {
        let get_soundboard_url = format!("http://{}:{}/v1/soundboard", self.ip, self.port);

        let response = self
            .client
            .get(get_soundboard_url)
            .send()
            .await?
            .json::<soundboard::SoundboardResponse>()
            .await?;

        Ok(response)
    }

    pub async fn get_playlist(&self) -> Result<playlist::PlaylistResponse, reqwest::Error> {
        let get_playlist_url = format!("http://{}:{}/v1/playlist", self.ip, self.port);

        let response = self
            .client
            .get(get_playlist_url)
            .send()
            .await?
            .json::<playlist::PlaylistResponse>()
            .await?;

        Ok(response)
    }
}
