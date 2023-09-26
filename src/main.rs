#![allow(unused)]

///! kenku_control
///!
///! A library for manage your Kenku FM using remote acess.
mod lib;
use lib::Controller;

use std::thread;
use std::time::Duration;

extern crate kenku_control;

#[tokio::main]
async fn main() {
    let ip = "127.0.0.1".to_string();
    let port = "3333".to_string();

    let controller = Controller::new(ip, port);

    let playlist = controller.get_soundboard().await.unwrap();
    let tracks = playlist.sounds;

    for track in tracks {
        track.play(&controller).await;
    }
}
