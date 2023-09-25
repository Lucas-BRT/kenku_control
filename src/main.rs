//#![allow(unused)]

///! kenku_control
///!
///! A library for manage your Kenku FM using remote acess.
use lib::Controller;
pub mod lib;


extern crate kenku_control;

#[tokio::main]
async fn main() {
    let ip = "127.0.0.1".to_string();
    let port = "3333".to_string();

    let kenku_controller = Controller::new(ip, port);
}
