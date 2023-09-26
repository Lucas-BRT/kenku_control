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

    let kenku_controller = Controller::new(ip, port);

    kenku_controller.get_playlist().await.unwrap().tracks[0]
        .play(&kenku_controller)
        .await
        .unwrap();

    lib::playback_repeat(&kenku_controller, lib::playlist::Repeat::Playlist)
        .await
        .unwrap();

    thread::sleep(Duration::from_secs(2));

    lib::playback_repeat(&kenku_controller, lib::playlist::Repeat::Track)
        .await
        .unwrap();

    thread::sleep(Duration::from_secs(2));

    lib::playback_repeat(&kenku_controller, lib::playlist::Repeat::Off)
        .await
        .unwrap();
}
