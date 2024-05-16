use super::*;
use std::net::TcpStream;

/// Checks the state of the Kenku server.
///
/// This function takes an IP address and a port, makes a GET request to the server, and returns the server's state.
///
/// # Arguments
///
/// * `ip` - A string slice that holds the IP address of the server.
/// * `port` - A 16 unsigned byte that holds the port number of the server.
/// * `delay_in_milisseconds` - A 16 unsigned byte that holds the time in milisseconds for the connection timeout.
///
/// # Returns
///
/// This function returns a `KenkuState` that represents the state of the server. If the GET request is successful, it returns `KenkuState::Online`. If the GET request fails, it returns `KenkuState::Offline`.
pub async fn check_kenku_server_state(address: SocketAddrV4) -> KenkuState {
    let is_server_online = TcpStream::connect(address).is_ok();

    if !is_server_online {
        return KenkuState::Offline;
    }

    KenkuState::Online
}

/// Create a base url pathern to Kenku Remote
///
/// This function takes an IP address and a port, and return a String containing the link for the Kenku Remote server
///
/// # Arguments
///
/// * `ip` - A string slice that holds the ip addres of the server.
/// * `port` - A unsigned 16 bit that hlds the port of the server.
///
/// # Returns
///
/// This function returns a `String` that contains the link containing the kenku remote ip and path
pub fn format_base_url(ip: String, port: u16) -> String {
    format!("http://{}:{}/v1", ip, port)
}

/// Processes a GET command for the Kenku Remote server.
///
/// This function takes a `KenkuGetCommand`, an IP address, and a port, and returns
/// a `String` containing the corresponding URL for the command on the Kenku Remote server.
///
/// # Arguments
///
/// * `command` - A `KenkuGetCommand` enum representing the type of GET command to process.
/// * `base_url` - A string slice containg the url of the kenku remote
///
/// # Returns
///
/// Returns a `String` containing the formatted URL corresponding to the command on the Kenku Remote server.
pub fn process_get_command(command: &KenkuGetCommand, base_url: &str) -> String {
    match command {
        KenkuGetCommand::Soundboard => format!("{}/soundboard", base_url),
        KenkuGetCommand::SoundboardPlayback => format!("{}/soundboard/playback", base_url),
        KenkuGetCommand::Playlist => format!("{}/playlist", base_url),
        KenkuGetCommand::PlaylistPlayback => format!("{}/playlist/playback", base_url),
    }
}

/// Processes a PUT command for the Kenku Remote server.
///
/// This function takes a `KenkuPutCommand`, an IP address, and a port, and returns
/// a `String` containing the corresponding URL for the command on the Kenku Remote server.
///
/// # Arguments
///
/// * `command` - A `KenkuPutCommand` enum representing the type of PUT command to process.
/// * `base_url` - A string slice containg the url of the kenku remote
///
/// # Returns
///
/// Returns a `String` containing the formatted URL corresponding to the command on the Kenku Remote server.
pub fn process_put_command(command: &KenkuPutCommand, base_url: &str) -> String {
    match command {
        KenkuPutCommand::PlaylistPlay => format!("{}/playlist/play", base_url),
        KenkuPutCommand::PlaylistPlaybackMute => format!("{}/playlist/playback/mute", base_url),
        KenkuPutCommand::PlaylistPlaybackPause => format!("{}/playlist/playback/pause", base_url),
        KenkuPutCommand::PlaylistPlaybackPlay => format!("{}/playlist/playback/play", base_url),
        KenkuPutCommand::PlaylistPlaybackRepeat => format!("{}/playlist/playback/repeat", base_url),
        KenkuPutCommand::PlaylistPlaybackShuffle => {
            format!("{}/playlist/playback/shuffle", base_url)
        }
        KenkuPutCommand::PlaylistPlaybackVolume => format!("{}/playlist/playback/volume", base_url),
        KenkuPutCommand::SoundboardPlay => format!("{}/soundboard/play", base_url),
        KenkuPutCommand::SoundboardStop => format!("{}/soundboard/stop", base_url),
    }
}

/// Processes a POST command for the Kenku Remote server.
///
/// This function takes a `KenkuPostCommand`, an IP address, and a port, and returns
/// a `String` containing the corresponding URL for the command on the Kenku Remote server.
///
/// # Arguments
///
/// * `command` - A `KenkuPostCommand` enum representing the type of POST command to process.
/// * `base_url` - A string slice containg the url of the kenku remote
///
/// # Returns
///
/// Returns a `String` containing the formatted URL corresponding to the command on the Kenku Remote server.
pub fn process_post_command(command: &KenkuPostCommand, base_url: &str) -> String {
    match command {
        KenkuPostCommand::PlaylistPlaybackNext => format!("{}/playlist/playback/next", base_url),
        KenkuPostCommand::PlaylistPlaybackPrevious => {
            format!("{}/playlist/playback/previous", base_url)
        }
    }
}

/// Constructs a URL for a given command, IP address, and port.
///
/// This function takes a `KenkuGetCommand`, an IP address, and a port, and returns a URL that can be used to make a GET request to the soundboard or playlist API.
///
/// # Arguments
///
/// * `command` - A reference to a `KenkuGetCommand` enum, which specifies the type of request to make.
/// * `ip` - A string slice that holds the IP address of the server.
/// * `port` - A string slice that holds the port number of the server.
///
/// # Returns
///
/// This function returns a `String` that represents the constructed URL.
pub fn process_url(command: &KenkuCommand, address: SocketAddrV4) -> String {
    let base_url = format_base_url(address.ip().to_string(), address.port());

    match command {
        KenkuCommand::KenkuGet(get_command) => process_get_command(get_command, base_url.as_str()),
        KenkuCommand::KenkuPut(put_command) => process_put_command(put_command, base_url.as_str()),
        KenkuCommand::KenkuPost(post_command) => {
            process_post_command(post_command, base_url.as_str())
        }
    }
}
