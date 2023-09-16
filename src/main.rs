use kenku_control::Controller;

#[tokio::main]
async fn main() {
    let ip = "127.0.0.1".to_string();
    let port = "3333".to_string();

    let kenku_controler = Controller::new(ip, port);

    let response = kenku_controler.get_soundboard().await.unwrap();

    println!("{:#?}", response.get_sounds());
}
