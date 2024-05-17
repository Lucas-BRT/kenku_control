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
kenku_control = "0.2.1"
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
    let controller = Controler::new("127.0.0.1".to_string(), 3333);

    let soundboards = controller
        .get_soundboard()
        .await
        .expect("failed to get kenku soundboards");
    // play all sounds in your soundboards
    for sound in soundboards.sounds {
        sound.play(&controller).await.unwrap();
    };
}
```

Make sure you have a Kenku Remote Online in your computer before running the code.

## Contributing

Contributions are welcome! If you have any ideas, suggestions, or bug reports, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

