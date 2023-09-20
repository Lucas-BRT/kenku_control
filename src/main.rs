#![allow(unused)]
///! # kenku_control
///!
///! A library for manage your Kenku FM using remote acess.
pub mod controller;
pub mod playlist;
pub mod soundboard;

#[tokio::main]
async fn main() {
    let ip = "127.0.0.1".to_string();
    let port = "3333".to_string();
}
