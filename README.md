# Kenku Control

[![Build Status](https://img.shields.io/travis/Lucas-BRT/kenku_control/master.svg)](https://travis-ci.org/Lucas-BRT/kenku_control)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/Lucas-BRT/kenku_control/blob/master/LICENSE)

Kenku Control is a library for controlling Kenku FM. It provides a set of functions and utilities to interact with the Kenku Remote.

## Features

- List all tracks and sounds in your kenku.
- Control the playback state.
- individual controll of the medias and the folders.

## Usage

Here's an example of how to use Kenku Control in your Rust code:

```
#[tokio::main]
async fn main() {
    let ip = "127.0.0.1".to_string();
    let port = "3333".to_string();

    let controller = Controller::new(ip, port);
    let playlists = controller.get_playlist().await.unwrap();
    
    for track in playlists.tracks {
        let status = track.play(&controller).await.unwrap();
        println!("{status:#?}");
    }
}

```

Make sure you have a Kenku Remote Online in your computer before running the code.

## Contributing

Contributions are welcome! If you have any ideas, suggestions, or bug reports, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

