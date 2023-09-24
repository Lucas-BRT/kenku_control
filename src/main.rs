#![allow(unused)]

use controller::Controller;
///! kenku_control
///!
///! A library for manage your Kenku FM using remote acess.
pub mod controller;
pub mod playlist;
pub mod soundboard;
mod tests;

#[tokio::main]
async fn main() {
    let ip = "127.0.0.1".to_string();
    let port = "3333".to_string();

    let kenku_controller = controller::Controller::new(ip, port);

    //    let a = kenku_controller.get_soundboard().await.unwrap();

    //    println!("{a:#?}");
}
