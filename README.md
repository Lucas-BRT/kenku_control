# Kenku Control

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/Lucas-BRT/kenku_control/blob/master/LICENSE)

Kenku Control is a library for controlling Kenku FM. It provides a set of functions and utilities to interact with the Kenku Remote.

## Features

- Retrieve information about all tracks and sounds available on your Kenku FM device.
- Control playback of tracks and sounds.
- Manage playlist playback using straightforward commands.

## Usage

To use Kenku Control in your Rust project, add the following to your Cargo.toml file:

```toml
[dependencies]
kenku_control = "0.1.2"
```

Alternatively, you can use cargo add to automatically manage dependencies:

```
cargo add kenku_control
```


Here's an example demonstrating how to interact with Kenku in your Rust code:

```rust
use kenku_control::Controller;

#[tokio::main]
async fn main() {
    let ip = "127.0.0.1";
    let port = "3333";

    let controller = Controller::new(ip, port);

    let tracks = match controller.get_playlist().await.unwrap().tracks;

    for track in tracks {
        track.play(&controller).await.unwrap();
    }
}
```

Make sure you have a Kenku Remote Online in your computer before running the code.

## Contributing

Contributions are welcome! If you have any ideas, suggestions, or bug reports, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

