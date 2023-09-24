#[cfg(test)]
mod kenku_remote_tests {
    use super::*;
    use crate::controller::{self, check_kenku_server_state, KenkuState};

    const IP: &str = "127.0.0.1";
    const PORT: &str = "3333";

    #[tokio::test]
    async fn test_kenku_server_state() {
        let kenku_state = check_kenku_server_state(&IP.to_string(), &PORT.to_string()).await;
    }

    #[tokio::test]
    async fn test_playlist_test() {}
}
