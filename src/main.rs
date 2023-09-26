#![allow(unused)]

///! kenku_control
///!
///! A library for manage your Kenku FM using remote acess.

mod lib;
use lib::Controller;


fn main() {
    let ip = "127.0.0.1".to_string();
    let port = "3333".to_string();

    let controller = Controller::new(ip, port);
}